pub trait Notifier {
    fn send_alert(&self, user_id: &str, message: &str) -> Result<(), String>;
}

pub struct EventProcessor<N: Notifier> {
    notifier_service: N, // Depends on the Notifier trait
    admin_user_id: String,
}

impl<N: Notifier> EventProcessor<N> {
    pub fn new(notifier_service: N, admin_user_id: String) -> Self {
        EventProcessor {
            notifier_service,
            admin_user_id,
        }
    }

    pub fn process_critical_event(&self, event_details: &str) {
        println!("Processing critical event: {}", event_details);
        let alert_message = format!("CRITICAL: {}", event_details);

        match self
            .notifier_service
            .send_alert(&self.admin_user_id, &alert_message)
        {
            Ok(_) => println!("Admin notified successfully."),
            Err(e) => println!("Failed to notify admin: {}", e),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*; // Import Notifier, EventProcessor

    // Stub implementation for the Notifier trait
    struct StubEmailNotifier {
        // Extra fields to control stub's behavior for different tests
        should_succeed: bool,
        expected_user_id: String,
        expected_message_contains: String,
        call_count: std::cell::Cell<usize>, // To track calls (simple mock-like behaviour)
    }

    impl Notifier for StubEmailNotifier {
        fn send_alert(&self, user_id: &str, message: &str) -> Result<(), String> {
            self.call_count.set(self.call_count.get() + 1); // Increment call count
            println!(
                "STUB: Attempting to send alert to '{}' with message '{}'",
                user_id, message
            );
            assert_eq!(
                user_id, self.expected_user_id,
                "Stub called with wrong user_id"
            );
            assert!(
                message.contains(&self.expected_message_contains),
                "Stub message content mismatch"
            );

            if self.should_succeed {
                Ok(())
            } else {
                Err("StubNotifier: Simulated failure".to_string())
            }
        }
    }

    #[test]
    fn critical_event_notifies_admin_successfully() {
        let stub_notifier = StubEmailNotifier {
            should_succeed: true,
            expected_user_id: "admin_001".to_string(),
            expected_message_contains: "CRITICAL: System Overload".to_string(),
            call_count: std::cell::Cell::new(0),
        };

        let event_processor = EventProcessor::new(stub_notifier, "admin_001".to_string());
        event_processor.process_critical_event("System Overload");

        // Check if the notifier was called (accessing our stub's field)
        // This make our stub act a bit like a mock.
        assert_eq!(
            event_processor.notifier_service.call_count.get(),
            1,
            "Notifier should have been called once"
        );
    }

    #[test]
    fn critical_event_handles_notification_failure() {
        let stub_notifier = StubEmailNotifier {
            should_succeed: false,
            expected_user_id: "sys_alert_user".to_string(),
            expected_message_contains: "CRITICAL: Disk Full".to_string(),
            call_count: std::cell::Cell::new(0),
        };

        let event_processor = EventProcessor::new(stub_notifier, "sys_alert_user".to_string());
        // no panic assertion, only that the function runs and internally
        // would print the failure message.
        // A more robust test might have process_critical_event return a Result.
        event_processor.process_critical_event("Disk Full");
        assert_eq!(
            event_processor.notifier_service.call_count.get(),
            1,
            "Notifier should have been called once, even on failure path"
        );
    }
}
