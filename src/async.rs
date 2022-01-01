use async_std::task;
use rand::prelude::*;
use std::{collections::HashMap, time::Duration};
#[allow(dead_code)]

async fn say_hello() {
    println!("Hello, world!");
}

async fn take_your_time(number: u32) -> (u32, u64) {
    let ms: u64 = rand::thread_rng().gen_range(1..100);
    task::sleep(Duration::from_millis(ms)).await;
    return (number, ms);
}

async fn broker(map: &mut HashMap<u32, u64>) {
    for n in 1..9 {
        map.insert(n, 0);
        let fut = take_your_time(n);
        let (task, duration) = fut.await;
        map.entry(task).and_modify(|e| *e = duration);
    }
    return;
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::collections::HashMap;

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
        let mut tasks = HashMap::new();
        task::block_on(broker(&mut tasks));
        let first = tasks.get(&1).unwrap();
        assert!(first > &0, "did not get a value on first");
        for (num, dur) in tasks {
            println!("Task {} took {} ms.", num, dur);
        }
    }
}
