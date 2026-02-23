use std::cell::RefCell;
use std::rc::Rc;

trait Observer {
    fn update(&self, new_state: &str);
}

struct Subject {
    state: String,
    observers: RefCell<Vec<Rc<dyn Observer>>>,
}

impl Subject {
    fn new(initial_state: &str) -> Self {
        Subject {
            state: initial_state.to_string(),
            observers: RefCell::new(Vec::new()),
        }
    }

    fn attach(&self, observer: Rc<dyn Observer>) {
        self.observers.borrow_mut().push(observer);
    }

    fn set_state(&mut self, new_state: &str) {
        println!(
            "\nSubject: State changed from '{}' to '{}'. Notifying observers...",
            self.state,
            new_state.to_string()
        );

        self.state = new_state.to_string();

        for observer in self.observers.borrow().iter() {
            observer.update(&self.state);
        }
    }
}

struct Logger {
    name: String,
}

impl Observer for Logger {
    fn update(&self, new_state: &str) {
        println!(
            "[Logger {}]: Received update! New state is: {}",
            self.name, new_state
        );
    }
}

struct Notifier {
    email: String,
}

impl Observer for Notifier {
    fn update(&self, new_state: &str) {
        println!(
            "[Notifier]: Sending email to {}. Subject: State changed to '{}'",
            self.email, new_state
        );
    }
}

fn main() {
    let mut subject = Subject::new("Initial State");

    let logger = Rc::new(Logger {
        name: "FileLog".to_string(),
    });

    let notifier = Rc::new(Notifier {
        email: "myemail@example.gom".to_string(),
    });

    subject.attach(Rc::clone(&logger) as Rc<dyn Observer>);
    subject.attach(Rc::clone(&notifier) as Rc<dyn Observer>);

    subject.set_state("State A");
    subject.set_state("State B");
}
