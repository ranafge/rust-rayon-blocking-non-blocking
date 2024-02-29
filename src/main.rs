use std::time::Duration;

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

// #[tokio::main]
// async fn main() {
//     // tokio::join!(
//     //     sleep_then_print(1),
//     //     sleep_then_print(2),
//     //     sleep_then_print(3)
//     // );

//     let blocking_task: tokio::task::JoinHandle<()> = tokio::task::spawn_blocking(|| {
//         println!("inside spawn locking");
//     });
//     blocking_task.await.unwrap();
// }


async fn sleep_then_print(timer: u32) {
    println!("Start timer {}", timer);
    tokio::time::sleep(Duration::from_secs(1)).await;
    println!("Timer {} done", timer);
}


async fn parallel_sum(nums: Vec<i32>) -> i32{;
    let (send, recv) = tokio::sync::oneshot::channel();
    //Spawn a task on rayon
    rayon::spawn(move || {
        // let mut sum = 0;
        // for num in nums {
        //     sum += num
        // }
        // Compute the sum on multiple threads
        let sum = nums.par_iter().sum();
        let _ = send.send(sum);
    });

    recv.await.expect("Panic in rayon::spawn")
}

#[tokio::main]
async fn main() {
    let nums = vec![1; 1024 * 1024];
    println!("{}", parallel_sum(nums).await);
}