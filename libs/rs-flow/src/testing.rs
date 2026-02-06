//! Testing helpers for unit-testing single components.
//!
//! Provides `Testing<T, V>` to build a test for a component type `T` that
//! implements `ComponentSchema<V>`. You can add input packages for input ports,
//! add globals that the component will be able to access, and run the
//! component's `run` method in isolation. The result is a `TestingResult`
//! containing the packages produced on the output ports and a serialized view
//! of the globals used.
//!
//! The module serializes only the test-visible data (inputs/outputs and the
//! globals as serde values). The component instance itself is not serialized.
use std::collections::{HashMap, VecDeque};

use serde::ser::{SerializeMap, Serializer};
use serde::Serialize;
use serde_json::Value as JsonValue;
use std::sync::Arc;

use crate::component::Component;
use crate::context::{Ctx, Global};
use crate::error::RunResult;
use crate::ports::{Inputs, Port, PortId};

/// A testing builder for a single component.
///
/// Usage sketch:
/// ```ignore
/// // assuming `MyComp` implements `ComponentSchema<MyPackage>`
/// let mut testing = Testing::new(MyComp::default());
/// testing
///     .input(MyInPort, pkg1)
///     .input(MyInPort, pkg2)
///     .global(SharedState { ... });
///
/// let result = testing.test().await.unwrap();
/// // inspect result.outputs and result.globals
/// ```
pub struct Testing<T, V>
where
    T: crate::component::ComponentSchema<V>,
{
    /// The component instance to test (moved into the test run).
    component: T,

    /// Map of input port id -> list of packages to feed into that input.
    inputs: HashMap<PortId, Vec<V>>,

    /// The context `Global` structure populated as `.global(...)` is called.
    context_global: Global,

    /// Serializable view of globals keyed by type name. This is what will be
    /// serialized from `Testing` / `TestingResult`. Stored as serde_json::Value
    /// so complex types can be serialized.
    serialized_globals: HashMap<String, JsonValue>,
}

impl<T, V> Testing<T, V>
where
    T: crate::component::ComponentSchema<V>,
{
    /// Create a new testing instance for the given component value.
    ///
    /// You provide the concrete component instance here. (This avoids requiring
    /// `T: Default`.)
    pub fn new(component: T) -> Self {
        Self {
            component,
            inputs: HashMap::new(),
            context_global: Global::default(),
            serialized_globals: HashMap::new(),
        }
    }

    /// Add a package `value` to the given input `port`.
    ///
    /// By default `input` uses the component's associated `T::Inputs` type,
    /// so you can call `testing.input(T::Inputs::SomeInput, value)`.
    pub fn input(mut self, port: T::Inputs, value: V) -> Self
    where
        T::Inputs: crate::ports::Inputs,
    {
        let port_id = port.into_port();
        self.inputs.entry(port_id).or_default().push(value);
        self
    }

    /// Add a package `value` to the given input by `PortId`.
    ///
    /// This method allows feeding inputs by port identifier directly:
    /// `testing.input_port_id(0, value)`.
    pub fn input_port_id(mut self, port_id: PortId, value: V) -> Self {
        self.inputs.entry(port_id).or_default().push(value);
        self
    }

    /// Add a package `value` to the given input by `Port` struct.
    ///
    /// Convenience wrapper that extracts the `PortId` from a `Port`.
    pub fn input_port(self, port: Port, value: V) -> Self {
        self.input_port_id(port.port, value)
    }

    /// Add a typed global `g` to the testing context. The value will be
    /// available to the component when it runs via `Ctx::with` / `Ctx::with_mut`.
    ///
    /// The global value type `G` must implement `Serialize` so we can produce a
    /// serialized representation for the test result. `G` must also satisfy
    /// the `Any + Send + Sync + 'static` bounds required by the runtime `Global`.
    pub fn global<G>(mut self, g: G) -> Self
    where
        G: serde::Serialize + std::any::Any + Send + Sync + 'static,
    {
        // keep a serialized copy for test output/inspection
        if let Ok(v) = serde_json::to_value(&g) {
            let type_name = std::any::type_name::<G>().to_string();
            self.serialized_globals.insert(type_name, v);
        } else {
            let type_name = std::any::type_name::<G>().to_string();
            self.serialized_globals.insert(type_name, JsonValue::Null);
        }

        // actually add to the execution Global so the component can access it
        self.context_global = self.context_global.add(g);

        self
    }

    /// Run the component's `run` method once with the provided inputs and
    /// globals. Returns a `TestingResult` with the outputs produced and the
    /// serialized globals map.
    ///
    /// Note: this executes the component in the same way the Flow does for a
    /// single component: a `Ctx` is created for it and its `run` is awaited.
    pub async fn test(self) -> RunResult<TestingResult<V>>
    where
        // bounds needed to construct `Component::new` and to place values into queues
        V: Send + Clone + 'static,
        T: crate::component::ComponentSchema<V>,
    {
        // Build a `Component<V>` so we can create a Ctx for it.
        let component = Component::new(1, self.component);

        // Create Arc<Global> as required by Ctx::from
        let global_arc = Arc::new(self.context_global);

        // Create context (Ctx) for this component
        let mut ctx = Ctx::from(&component, Arc::clone(&global_arc));

        // Populate receive queues with the provided input packages
        for (port, vec_values) in self.inputs {
            // Convert Vec<V> -> VecDeque<V> and push into ReceiveQueue via push_all
            let mut deque = VecDeque::from(vec_values);
            // The Ctx.receive map uses PortId; we must find the receive queue and push
            if let Some(queue) = ctx.receive.get_mut(&port) {
                queue.push_all(&mut deque);
            } else {
                // If port not found the production should mirror the library behavior:
                // ctx.receive would panic on a non-existent port, but we return an error
                // as crate Error so user can see the mismatch.
                return Err(Box::new(crate::error::Error::InPortNotFound {
                    component: ctx.id,
                    in_port: port,
                }));
            }
        }

        // Run the component. Use the component.boxed impl that Flow uses.
        // component.data.run expects &mut Ctx<V>
        let run_result = component.data.run(&mut ctx).await?;

        let Ctx { send, global, .. } = ctx;
        // After run, collect outputs from ctx.send (each output port -> Vec<V>)
        let mut outputs: HashMap<PortId, Vec<V>> = HashMap::new();
        for (port, queue) in send.into_iter() {
            // queue is VecDeque<V>, convert to Vec<V>
            let vec: Vec<V> = queue.into();
            outputs.insert(port, vec);
        }

        // Build TestingResult
        // drop the context so the Arc<Global> held by `ctx` is released and we can
        // unwrap the original Arc into the owned Global value.
        drop(global);

        let global = Arc::try_unwrap(global_arc)
            .expect("Global no have multiples references, because ctx already dropped");

        let result = TestingResult {
            outputs,
            globals: self.serialized_globals,
            // we also include the `Next` value returned by the component run so
            // tests can assert whether the component requested to continue/break.
            next: run_result,
            global,
        };

        Ok(result)
    }
}

/// The result of running a single-component test.
///
/// - `outputs` maps output port id to the list of packages produced on that
///   port during the run.
/// - `globals` holds serialized representations of the globals that were
///   provided to the component (keyed by the concrete type name).
/// - `next` is the `Next` value produced by the component's `run`
///   invocation (Continue/Break).
pub struct TestingResult<V> {
    pub outputs: HashMap<PortId, Vec<V>>,
    /// Serialized view of globals (type name -> json value)
    pub globals: HashMap<String, JsonValue>,
    /// The runtime Global bag returned after execution (owned)
    pub global: Global,
    pub next: crate::component::Next,
}

impl<V> TestingResult<V> {
    /// Access a typed global immutably by running the provided closure with a
    /// reference to the stored global if present.
    pub fn with_global<T, F, R>(&self, f: F) -> Option<R>
    where
        T: std::any::Any + Send + Sync + 'static,
        F: FnOnce(&T) -> R,
    {
        self.global.with(f)
    }

    /// Access a typed global mutably by running the provided closure with a
    /// mutable reference to the stored global if present.
    pub fn with_global_mut<T, F, R>(&self, f: F) -> Option<R>
    where
        T: std::any::Any + Send + Sync + 'static,
        F: FnOnce(&mut T) -> R,
    {
        self.global.with_mut(f)
    }

    /// Remove and return a typed global from the bag. Consumes the stored
    /// global value if present.
    pub fn remove_global<T>(&mut self) -> Option<T>
    where
        T: std::any::Any + Send + Sync + 'static,
    {
        self.global.remove::<T>()
    }

    // --- Utility getters for outputs ---

    /// Return a reference to the entire outputs map.
    pub fn outputs_map(&self) -> &HashMap<PortId, Vec<V>> {
        &self.outputs
    }

    /// Return the output values for a given port as a slice, if present.
    pub fn get_output_slice(&self, port: PortId) -> Option<&[V]> {
        self.outputs.get(&port).map(|v| v.as_slice())
    }

    /// Return the output Vec for a given port, if present.
    pub fn get_output_vec(&self, port: PortId) -> Option<&Vec<V>> {
        self.outputs.get(&port)
    }

    /// Return the first value on the given output port, if any.
    pub fn get_first_output(&self, port: PortId) -> Option<&V> {
        self.outputs.get(&port).and_then(|v| v.first())
    }

    /// Return the only value on the given port, if and only if the port has exactly one item.
    pub fn get_single_output(&self, port: PortId) -> Option<&V> {
        self.outputs
            .get(&port)
            .and_then(|v| if v.len() == 1 { v.get(0) } else { None })
    }

    /// Return a serialized global value (serde_json) by its type name string, if present.
    pub fn get_serialized_global(&self, type_name: &str) -> Option<&JsonValue> {
        self.globals.get(type_name)
    }

    // --- Assertion helpers (panic on mismatch) ---

    /// Assert that the output for `port` is equal to `expected` (by slice).
    /// Panics with `assert_eq!` if not equal.
    pub fn assert_output_eq(&self, port: PortId, expected: &[V])
    where
        V: PartialEq + std::fmt::Debug,
    {
        let actual = self.get_output_slice(port).unwrap_or(&[]);
        assert_eq!(actual, expected, "output for port {} did not match", port);
    }

    /// Assert that the output Vec for `port` is exactly `expected_vec`.
    pub fn assert_output_vec_eq(&self, port: PortId, expected_vec: &Vec<V>)
    where
        V: PartialEq + std::fmt::Debug,
    {
        let actual = self
            .get_output_vec(port)
            .expect(&format!("{port} not found in tested component"));

        assert_eq!(
            actual, expected_vec,
            "output vec for port {} did not match",
            port
        );
    }

    /// Assert that the output for `port` has length `expected_len`.
    pub fn assert_output_len(&self, port: PortId, expected_len: usize) {
        let len = self
            .get_output_slice(port)
            .expect(&format!("{port} not found in tested component"))
            .len();

        assert_eq!(
            len, expected_len,
            "output length for port {} mismatch",
            port
        );
    }

    /// Assert that a given value is contained in the output list for `port`.
    pub fn assert_output_contains(&self, port: PortId, value: &V)
    where
        V: PartialEq + std::fmt::Debug,
    {
        let present = self
            .get_output_vec(port)
            .expect(&format!("{port} not found in tested component"))
            .contains(value);

        assert!(
            present,
            "expected value not found in outputs for port {}",
            port
        );
    }

    /// Assert that the port has exactly one value and that it equals `expected`.
    pub fn assert_single_output_eq(&self, port: PortId, expected: &V)
    where
        V: PartialEq + std::fmt::Debug,
    {
        let single = self
            .get_single_output(port)
            .expect("expected single output value");

        assert_eq!(
            single, expected,
            "single output for port {} did not match",
            port
        );
    }
}

/// Custom Serialize for `Testing<T, V>`.
///
/// We intentionally do not serialize the concrete component instance;
/// instead we serialize the user-visible inputs and the serialized globals map.
impl<T, V> Serialize for Testing<T, V>
where
    T: crate::component::ComponentSchema<V> + Serialize,
    V: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Serialize as map { "component": <serialized component or type>, "inputs": { port: [..] }, "globals": { type_name: value } }
        let mut map = serializer.serialize_map(Some(3))?;
        // try to serialize the concrete component value; if that fails fall back to the type name
        let comp_entry = serde_json::to_value(&self.component)
            .unwrap_or(JsonValue::String(std::any::type_name::<T>().to_string()));
        map.serialize_entry("component", &comp_entry)?;
        map.serialize_entry("inputs", &self.inputs)?;
        map.serialize_entry("globals", &self.serialized_globals)?;
        map.end()
    }
}

/// Derive Serialize for TestingResult where V: Serialize
impl<V> Serialize for TestingResult<V>
where
    V: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(3))?;
        map.serialize_entry("outputs", &self.outputs)?;
        map.serialize_entry("globals", &self.globals)?;
        map.serialize_entry("next", &self.next)?;
        map.end()
    }
}
