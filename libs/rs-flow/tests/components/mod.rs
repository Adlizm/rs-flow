use rs_flow::prelude::Global;

pub mod log;
pub mod message;

#[derive(Debug)]
pub struct CounterLogs {
    pub count: i32,
}
impl Global for CounterLogs {
    type Package = String;
}
