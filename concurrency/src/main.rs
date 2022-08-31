use std::thread;
use std::sync::mpsc;
use std::time::Duration;
use std::sync::Mutex;

fn threads() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here is a vector: {:?}", v);
    });

    handle.join().unwrap();
}

fn messages() {
    let (tx, rx) = mpsc::channel();

    let tx1 = mpsc::Sender::clone(&tx);
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // recv method is blocking, whereas try_recv is not
    // let received = rx.recv().unwrap();
    for received in rx {
        println!("Got: {}", received);
    }
}

fn main() {
    // threads();
    // messages();

    // Create Mutex<i32>
    // Mutex is a smart pointer
    let m = Mutex::new(5);

    {
        // We have to lock the mutex before access to the data
        // lock method return a MutexGuard
        let mut num = m.lock().unwrap();
        *num = 6;
    }
    // As Mutex is a smart pointer, the lock is dropped when the MutexGuard is out of scope

    println!("m = {:?}", m);
}
