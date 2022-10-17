use async_std::task::sleep;
use std::{thread, time::Duration};

use futures::{executor::block_on, join};

fn main() {
    block_on(main_async());
}

/*
    Threads:
    +   can change priority of threads + have lower latency
    -   cpu + memory overhead
    Processes:
    +   fewer expensive threads
    +   good for io bound tasks
    -   larger binary blobs
    -   reduced CPU and memory overhead,
*/

async fn main_async() {
    println!("Hello, world!");
    join!(
        not_main_1(),
        not_main_2(),
        thread::spawn(|| async {
            not_main_1().await;
        })
        .join()
        .unwrap()
    );
}

async fn not_main_1() {
    println!("waiting...");
    sleep(Duration::from_millis(1000)).await;
    println!("fn 1 output");
}

async fn not_main_2() {
    println!("fn 2 output");
}
