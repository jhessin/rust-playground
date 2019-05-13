use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(
                1,
            ));
        }
    });
    handle.join().unwrap(); // The main thread can't move on until handle
                            // finishes.
    for i in 1..5 {
        println!(
            "hi number {} from the main thread!",
            i
        );
        thread::sleep(Duration::from_millis(1));
    }

    // Moving data from one thread to another.
    let v = vec![1, 2, 3];

    // NOTE: Need to use move or this will throw a
    // compile time error.
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
}
