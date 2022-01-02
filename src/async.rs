use async_std::task;
use futures::{stream::FuturesUnordered, StreamExt};
use rand::prelude::*;
use std::{time::Duration};
#[allow(dead_code)]

async fn say_hello() {
    println!("Hello, world!");
}

async fn take_your_time(number: u32) -> (u32, u64) {
    let ms: u64 = rand::thread_rng().gen_range(1..100);
    task::sleep(Duration::from_millis(ms)).await;
    return (number, ms);
}

async fn broker(map: &mut Vec<(u32, u64)>) {
    let futures = FuturesUnordered::new();
    for n in 0..9 {
        futures.push(take_your_time(n));
    }
    let results: Vec<(u32, u64)> = futures.collect().await;
    for r in results {
        map.push(r);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hello() {
        task::block_on(say_hello())
    }
    #[test]
    fn spawn_single() {
        println!("spawn_single");
        let handle = task::spawn(take_your_time(0));
        println!("Spawned task {}.", 0);
        task::block_on(handle);
    }

    #[test]
    fn spawn_multiple() {
        println!("=== spawn_multiple ===");
        let mut tasks = Vec::new();
        task::block_on(broker(&mut tasks));
        let first = tasks.get(1).unwrap();
        assert!(first.1 > 0, "did not get a value on first");
        for (num, dur) in tasks {
            println!("Task {} took {} ms.", num, dur);
        }
    }
}
