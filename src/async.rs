use async_std::task;
use futures::{stream::FuturesUnordered, StreamExt};
use rand::prelude::*;
use std::{time::Duration};
#[allow(dead_code)]

async fn say_hello() {
    println!("Hello, world!");
}

async fn take_your_time(i: usize) -> (usize, u64) {
    let ms: u64 = rand::thread_rng().gen_range(1..100);
    task::sleep(Duration::from_millis(ms)).await;
    return (i, ms);
}

// Are you trying to select! the first future to complete?
// (and then loop until all completed)

// FuturesUnordered implements Stream so you can do while let Some(ret) = futures.next().await


async fn broker(tasks: &mut Vec<u64>) {
    let mut futures = FuturesUnordered::new();
    for (i,_) in tasks.iter().enumerate() {
        futures.push(take_your_time(i));
    }
    while let Some((i, d)) = futures.next().await{
        tasks[i] = d;
    }
}

// async fn static_broker(tasks: &'static mut Vec<u64>) {
//     let mut futures = FuturesUnordered::new();
//     for (i,_) in tasks.iter().enumerate() {
//         futures.push(take_your_time(i));
//     }
//     while let Some((i, d)) = futures.next().await{
//         tasks[i] = d;
//     }
// }

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn hello() {
        task::block_on(say_hello())
    }
    #[test]
    fn spawn_single() {
        let (_, result) = task::block_on(take_your_time(0));
        assert!(result > 0, "result was 0");
        println!("This took {result} ms.");
    }

    #[test]
    fn block_multiple_spawns() {
        let mut tasks = vec![0; 9];
        task::block_on(broker(&mut tasks));
        let first = tasks.get(1).unwrap();
        assert!(first > &0u64, "did not get a value on first");
        for (num, dur) in tasks.iter().enumerate() {
            println!("Task {} took {} ms.", num, dur);
        }
    }

    // #[test]
    // fn spawn_multiple_spawns() {
    //     let mut tasks = vec![0u64; 9];
    //     task::spawn(static_broker(&tasks));
    //     let first = tasks.get(1).unwrap();
    //     assert!(first > &0u64, "did not get a value on first");
    //     for (num, dur) in tasks.iter().enumerate() {
    //         println!("Task {} took {} ms.", num, dur);
    //     }
    // }

    #[test]
    fn spawn_communicating_tasks() {

    }
}
