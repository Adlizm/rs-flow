pub mod echo;
pub mod log;
pub mod message;

#[derive(Debug)]
pub struct CounterLogs {
    pub count: i32,
}

#[cfg(all(test, feature = "testing"))]
mod tests {
    use super::*;
    use crate::Testing;
    use tokio_test;

    // Test the Message component: it should produce a message on its output port
    #[test]
    fn message_component_test() {
        tokio_test::block_on(async {
            // Message component has no inputs and immediately sends its message
            let testing = Testing::new(message::Message::new("Hello"));
            let result = testing.test().await.unwrap();

            // Use TestingResult assertion helper
            result.assert_single_output_eq(0, &"Hello".to_string());
        });
    }

    // Test the Echo component: it should forward received inputs to its output
    #[test]
    fn echo_component_test() {
        tokio_test::block_on(async {
            let mut testing = Testing::new(echo::Echo);
            testing.input(echo::In::Data, "abc".to_string());
            let result = testing.test().await.unwrap();

            // Use TestingResult assertion helper
            result.assert_single_output_eq(0, &"abc".to_string());
        });
    }

    // Test the Log component: it should increment the CounterLogs global when receiving a message
    #[test]
    fn log_component_test() {
        tokio_test::block_on(async {
            let mut testing = Testing::new(log::Log);

            // Provide the CounterLogs global that the Log component will mutate
            testing.global(CounterLogs { count: 0 });

            // Feed a message into the Log input port
            testing.input(log::In::Message, "hey".to_string());

            let mut result = testing.test().await.unwrap();

            // Log does not emit outputs; assert the output length for port 0 is 0
            result.assert_output_len(0, 0);

            // Verify the global was mutated: CounterLogs.count should be 1
            let count_opt = result.with_global::<CounterLogs, _, _>(|c| c.count);
            assert_eq!(count_opt, Some(1));

            // Also demonstrate removing the global
            let removed: Option<CounterLogs> = result.remove_global::<CounterLogs>();
            assert!(removed.is_some());
            assert_eq!(removed.unwrap().count, 1);
        });
    }
}
