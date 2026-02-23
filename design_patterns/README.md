# Common desing patterns in Rust

## Builder pattern
- a **creational pattern** used to construct complex objects step-by-step while allowing optional configuration and sensible defaults. It improves readability, avoids long constructors with many parameters, and makes object creation more maintainable and flexible.

## Observer pattern
- a **behavioral design pattern** where objects (observers) subscribe to a subject and are automatically notified when its state changes. The Subject maintains a list of Observer trait objects using `Rc<dyn Observer>` and `RefCell` to allow shared ownership and interior mutability. When `set_state` is called, all attached observers are notified via polymorphic dispatch through the update method.

## State pattern
- a behavioral design pattern where an object changes its behavior based on its internal state. The Post struct delegates behavior to different state variants (`Draft`, `PendingReview`, `Published`) represented by an enum, enforcing valid state transitions at compile time. By modeling each state explicitly, the design prevents invalid operations (like adding text after review) and makes transitions controlled and predictable.

## Strategy pattern
- a behavioral design pattern that allows selecting an algorithm’s behavior at runtime. The `TextProcessor` delegates formatting logic to a `TextFormattingStrategy` trait object, enabling interchangeable formatting strategies such as uppercase or lowercase. By injecting the strategy via `Box<dyn Trait>`, the design achieves loose coupling and open/closed principle compliance.