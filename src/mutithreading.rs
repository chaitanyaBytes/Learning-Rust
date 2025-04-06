use std::{thread, time::Duration};

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {i} from spawned thread");
            thread::sleep(Duration::from_millis(1));
        }
    });

    // waits for the thread to finish and ond then starts the execution of the remaining code
    handle.join().unwrap();

    for i in 1..5 {
        println!("hi number {i} from main thread");
        thread::sleep(Duration::from_millis(1));
    }
}

use std::thread;

fn main() {
    let x = 1;

    // closure may outlive the current function, but it borrows `v`,
    // which is owned by the current function may outlive borrowed value `v`
    {
        let v = vec![1, 2, 3];
        thread::spawn(|| {
            println!("{:?}", v);
        });
    }

    // to force the closure to take ownership of `v` (and any other referenced variables), use the `move` keyword: `move `
    println!("{}", x);
}

// corrected code
use std::thread;

fn main() {
    let x = 1;

    {
        let v = vec![1, 2, 3];
        thread::spawn(move || {
            println!("{:?}", v);
        });
    }
    // move helps forcefully take the ownership of v from its scope to the closure

    println!("{}", x);
}
