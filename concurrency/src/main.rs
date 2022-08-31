use std::thread;
use std::sync::mpsc;

fn threads() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here is a vector: {:?}", v);
    });

    handle.join().unwrap();
}

fn main() {
    threads();

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    // recv method is blocking, whereas try_recv is not
    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}
