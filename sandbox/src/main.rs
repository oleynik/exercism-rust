use std::sync::mpsc;
use std::thread;

fn main() {
    let (send, receiver) = mpsc::channel();
    let mut handlers = vec![];

    for _ in 0..2 {
        let sender = send.clone();
        handlers.push(thread::spawn(move || {
            let thread_id = thread::current().id();
            sender.send(format!("Hello from {:?}", thread_id)).unwrap();
        }));
    }
    drop(send);

    println!("Ready for messages...");
    for m in receiver {
        println!("{:?} | Received: {}", thread::current().id(), m)
    }
    println!("Messages Read!");
}
