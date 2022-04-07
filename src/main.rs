use chrono;
use tokio;
use std::pin::Pin;
use std::time::Duration;

use async_demo::time_it;

/// Print a messages to stdout, and identify the thread that requested
/// the printing.
fn thread_rollcall(msg: &str) {
    println!("{:?}: {}", std::thread::current().id(), msg);
}

// This function is synchronous.
fn expensive_computation() -> u32 {
    thread_rollcall("About to wait 1 second.");
    std::thread::sleep(Duration::from_millis(1000));
    408
}

// This function is asynchronous.
async fn wait_async() -> u32 {
    thread_rollcall("About to async_wait");
    tokio::time::sleep(Duration::from_millis(1000)).await;
    thread_rollcall("Finished async_wait");
    1337
}

#[tokio::main]
async fn main() {
    println!("Starting.");
    // let res = wait_async().await;
    let (res,time) = time_it(|| Box::pin( wait_async()) ).await;
    println!("Hello, world! Result: {:?} in {:?}", res, time);
}


