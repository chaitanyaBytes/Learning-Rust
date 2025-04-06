use std::{sync::mpsc, thread};
// for sending data across multiple thread and main thread as well

fn main() {
    let (tx, rx) = mpsc::channel();

    for i in 0..10 {
        let producer = tx.clone();

        thread::spawn(move || {
            let mut sum: u64 = 0;
            for j in 1..10000000 {
                sum = sum + (i * 10000000 + j);
            }

            producer.send(sum).unwrap();
        });
    }
    drop(tx);
    // "Okay, I’m done sending from the main thread — no more messages will be sent."

    let mut ans: u64 = 0;
    for val in rx {
        ans += val;
        println!("recieved values");
    }

    println!("ans is {}", ans);
}
