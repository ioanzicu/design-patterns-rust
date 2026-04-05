use std::cell::RefCell;
use std::rc::Rc;

// A logger that uses interior mutability to track its state.
struct MessageLogger {
    message_count: RefCell<usize>,
    history: RefCell<Vec<String>>,
}

impl MessageLogger {
    fn new() -> Self {
        MessageLogger {
            message_count: RefCell::new(0),
            history: RefCell::new(Vec::new()),
        }
    }

    // This method takes &self but can modify internal state via RefCell.
    fn log(&self, message: &str) {
        let mut count = self.message_count.borrow_mut();
        *count += 1;
        self.history
            .borrow_mut()
            .push(format!("#{}: {}", *count, message));
    }

    fn get_count(&self) -> usize {
        *self.message_count.borrow()
    }

    fn print_history(&self) {
        println!("---   Log History   ---");
        for entry in self.history.borrow().iter() {
            println!("{}", entry);
        }
        println!("-----------------------");
    }
}

fn main() {
    // Wrap the MessageLogger in Rc to allow shared ownership.
    let shared_logger: Rc<MessageLogger> = Rc::new(MessageLogger::new());

    // Create multiple references to the same logger instance.
    let logger_clone1 = Rc::clone(&shared_logger);
    let logger_clone2 = Rc::clone(&shared_logger);

    // Log messages from different references.
    // Each call will mutate the same underlaying MessageLogger instance.
    shared_logger.log("Main logger event.");

    logger_clone1.log("Event from clone 1.");
    logger_clone2.log("Event from cloen 2.");

    // Check the final state from any of the references.
    println!("\nTotal messages logged: {}", shared_logger.get_count());
    shared_logger.print_history();
}
