use std::rc::Rc;

#[derive(Debug)]
struct SharedConfig {
    version: String,
    api_key: String,
}

struct ServiceA {
    id: u32,
    config: Rc<SharedConfig>,
}

struct ServiceB {
    name: String,
    config: Rc<SharedConfig>,
}

fn main() {
    // Create the shared configuration data wrapped in an Rc.
    let shared_config = Rc::new(SharedConfig {
        version: "v1.2.3".to_string(),
        api_key: "ABC123XYZ789".to_string(),
    });
    println!("Initial strong count: {}", Rc::strong_count(&shared_config)); // 1

    // Create clones of the Rc to share ownership.
    // This only increments the reference count; it does not deep copy the data.
    let service_a = ServiceA {
        id: 101,
        config: Rc::clone(&shared_config),
    };
    println!(
        "After ServiceA created: strong count = {}",
        Rc::strong_count(&shared_config)
    ); // 2

    let service_b = ServiceB {
        name: "LoggedService".to_string(),
        config: Rc::clone(&shared_config),
    };
    println!(
        "After ServiceB created: strong count = {}",
        Rc::strong_count(&shared_config)
    ); // 3

    println!("---");
    println!(
        "Service A accesses config version: {}",
        service_a.config.version
    );
    println!("Service B accesses API key: {}", service_b.config.api_key);
    println!("---");

    // Explicitly drop service_a to see the reference ccount decrease.
    drop(service_a);
    println!(
        "After ServiceA is dropped: strong count = {}",
        Rc::strong_count(&shared_config)
    ); // 2

    drop(service_b);
    println!(
        "After ServiceB is dropped: strong count = {}",
        Rc::strong_count(&shared_config)
    ); // 1

    // The final Rc (`shared_config`) will be dropped at the end of main's scope.
    // At that point, the count will become 0, and the SharedConfig data will be deallocated.
}
