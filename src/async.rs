use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    let f = my_fn();
    println!("Lets get rusty");
    f.await;
}

async fn my_fn() {
    println!("I am in an async function");
    let s1 = db_call().await;
    println!("{}", s1);
    let s2 = db_call().await;
    println!("{}", s2);
}

async fn db_call() -> String {
    sleep(Duration::from_secs(1)).await;
    return "Db Result".to_owned();
}

// handling the futures concurrently using spawn (similar to threads)
use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    let mut handles = vec![];

    for i in 0..2 {
        let handle = tokio::spawn(async move {
            my_fn(i).await;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.await.unwrap();
    }
}

async fn my_fn(i: i8) {
    println!("{i} I am in an async function");
    let s1 = db_call().await;
    println!("{i} first result: {s1}");
    let s2 = db_call().await;
    println!("{i} second result: {s2}");
}

async fn db_call() -> String {
    sleep(Duration::from_millis(100)).await;
    return "Db Result".to_owned();
}
