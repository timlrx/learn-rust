use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {i} from the main thread!");
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();

    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {v:?}");
    });

    handle.join().unwrap();
    // println!("vector {:?}", v); // error: move transfer ownership

    /* Channels */
    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone(); // Create a new transmitter

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(10));
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
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_millis(10));
        }
    });

    let received = rx.recv().unwrap();
    println!("Got: {received}");

    for received in rx {
        println!("Got: {received}");
    }

    /* Mutex */
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {m:?}");

    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap(); // Mutex provides internal mutability

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());

    /* Scoped threads */
    // Alternatively, we can use scoped threads to borrow from the main thread
    // Threads are guaranteed to complete before leaving the scope

    // Create some mutable data
    let mut data = vec![1, 2, 3, 4, 5];

    thread::scope(|s| {
        let mid = data.len() / 2;
        let (left, right) = data.split_at_mut(mid);

        // We can borrow data because the scope guarantees it won't outlive this block
        s.spawn(move || {
            // We can mutate the data safely because we have exclusive access to this slice
            for item in left.iter_mut() {
                *item *= 2;
            }
        });

        // Spawn another thread to work on the second half
        s.spawn(move || {
            for item in right.iter_mut() {
                *item *= 3;
            }
        });
    });

    println!("Modified data: {:?}", data);
}
