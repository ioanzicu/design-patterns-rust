use std::sync::mpsc;

fn main() {
    // Create a new channel. The type of data sent will be i32.
    let (tx, rx): (mpsc::Sender<i32>, mpsc::Receiver<i32>) = mpsc::channel();

    // tx is the Sender (transmitter)
    // rx is the Receiver
    println!("Channel created successfully! Sender: {:?}, Receiver: {:?}", tx, rx);
}