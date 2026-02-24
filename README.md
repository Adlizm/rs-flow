# rs-flow

System of flows in Rust.

You can see examples in `libs/rs-flow/tests` folder.

## Features

This crate exposes optional features to keep dependencies small by default:

- `serde` — enable `serde` + `serde_derive` support across types in the crate.
- `testing` — enables the testing helpers (`Testing`, `TestingResult`) and test utilities. This feature requires `serde` (it uses `serde_json` for the serialized view of globals).

Feature configuration is defined in `libs/rs-flow/Cargo.toml`. The `testing` feature is intended for unit / component tests and brings in the serialization helpers.

## Running tests

Tests that use the `testing` helpers are feature-gated. To run the tests that depend on `testing`, enable the feature when running `cargo test`. From the repository root you can run:

- Run all tests for the library with the `testing` feature:
```
cargo test -p rs-flow --no-default-features --features testing
```

- Run only the library tests (no binaries) with the `testing` feature:
```
cargo test -p rs-flow --no-default-features --features testing --lib
```

- Run a single test by name (example `log_component_test`):
```
cargo test -p rs-flow --no-default-features --features testing log_component_test
```

The component-focused tests live under:
```
libs/rs-flow/tests/components
```
They use the `Testing` builder for isolated component runs and the `TestingResult` helper methods for assertions.

## Examples & notes

- The `Testing` builder allows feeding inputs (by `T::Inputs` or by `PortId`) and registering globals that components can read/mutate.
- `Testing::test()` returns a `TestingResult` that includes:
  - `outputs` — produced packages for each output port,
  - `globals` — serialized view (JSON values) of provided globals,
  - `global` — the actual runtime `Global` bag (so tests can inspect or remove typed globals),
  - `next` — the `Next` enum returned by the component run.
- Use the provided `TestingResult` helpers such as `assert_single_output_eq`, `assert_output_len`, `with_global`, and `remove_global` to make tests concise and expressive.

## Roadmap / Next features
<ul>
  <li>[x] Update errors</li>
  <li>[x] Run flow async</li>
  <li>[x] Add global data to the flow, which can be accessed by components</li>
  <li>[x] Return Global when finish flow run, without cloned</li>
  <li>[x] Refactor Inputs and Outputs ports and Component struct and trait</li>
  <li>[x] Turn a workspace</li>
  <li>[x] Macros to implement Inputs and Outputs trait by component</li>
  <li>[x] Update for a queue by component, merge queues each cycle</li>
  <li>[x] Check if a connection creates a loop</li>
  <li>[x] Components can return { Continue or Break } to control flow execution</li>
  <li>[x] Create component types { Lazy or Eager } that define when a component will be executed</li>
  <li>[x] Check if a package has been consumed from queue (Loop detected)</li>
  <li>[x] Docs</li>
  <li>[x] Testing</li>
  <li>[x] Updating `ComponentSchema` trait</li>
  <li>[x] Refactor `Ctx` interfaces for more safer usage</li>
  <li>[ ] Create a features for optional run flow components in parallel</li>
</ul>
