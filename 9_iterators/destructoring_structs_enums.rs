enum Message {
    Quit,
    Write(String),
    ChangeColor(u8, u8, u8),
    Move { x: i32, y: i32 },
}

struct User {
    id: u32,
    name: String,
    active: bool,
}

fn process_message(msg: Message) {
    match msg {
        Message::Quit => println!("Quitting."),
        Message::Write(text) => println!("Message to write: {}", text),
        Message::ChangeColor(r, g, b) => println!("Change color to R:{} G:{} B:{}", r, g, b),
        Message::Move { x, y } => println!("Move to ({}, {})", x, y),
    }
}

fn describe_user(user: User) {
    match user {
        User {
            id: 1,
            name,
            active: true,
        } => println!("Admin user '{}' is active.", name),
        User { active: false, .. } => println!("User {} is inactive.", user.id),
        User { id, name, .. } => println!("Regular user #{} is '{}'.", id, name),
    }
}

fn main() {
    process_message(Message::ChangeColor(255, 0, 128));
    process_message(Message::Move { x: 10, y: -5 });

    let user1 = User {
        id: 1,
        name: "Alice".to_string(),
        active: true,
    };
    let user2 = User {
        id: 2,
        name: "Bob".to_string(),
        active: false,
    };
    let user3 = User {
        id: 3,
        name: "Charlie".to_string(),
        active: true,
    };

    describe_user(user1); // Admin user 'Alice' is active.
    describe_user(user2); // User Bob is inactive.
    describe_user(user3); // Regular user #3 is 'Charlie'.
}
