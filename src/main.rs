use tokio::runtime::Builder;
use tokio::spawn;

use std::env::args;
use std::time::SystemTime;

/// Baseline double-recursive fibonacci number calculation
fn fib(n: u64) -> u64 {
    if n <= 2 {
        return 1;
    }

    fib(n - 1) + fib(n - 2)
}

/// Entry point for the parallel Fibonacci. This splits the load between 2
/// tokio tasks
async fn fib_par_0(n: u64) -> u64 {
    if n <= 2 {
        return 1;
    }

    let v1_future = spawn(fib_par_1(n - 1));
    let v2_future = spawn(fib_par_1(n - 2));

    v1_future.await.unwrap() + v2_future.await.unwrap()
}

/// Intermediate Fibonacci parallel function. This splits the load again between
/// 2 more tokio tasks effectively brining the workers up to 4
async fn fib_par_1(n: u64) -> u64 {
    if n <= 2 {
        return 1;
    }

    let v1_future = spawn(fib_par_2(n - 1));
    let v2_future = spawn(fib_par_2(n - 2));

    v1_future.await.unwrap() + v2_future.await.unwrap()
}

/// Final stage of the Fibonacci parallel function. This splits the load one
/// final time to bring the total workers up to 8. All the remaining work will
/// be done using the standard Fibonacci calculation function
async fn fib_par_2(n: u64) -> u64 {
    if n <= 2 {
        return 1;
    }

    let v1_future = spawn(async move { fib(n - 1) });
    let v2_future = spawn(async move { fib(n - 2) });

    v1_future.await.unwrap() + v2_future.await.unwrap()
}

fn main() {

    // Parse the fibonacci number ordinal form the from command line
    let n : u64 = args()
        .skip(1) // skip the invocation
        .map(|n| n.parse::<u64>())
        .next()
        .expect("This program takes a single non-negative integer argument")
        .ok()
        .expect("Failed to parse first argument as an non-negative integer");

    // Run the single-threaded version
    {
        let start = SystemTime::now();
        let result = fib(n);

        println!("normal  : {:?} - {}", start.elapsed().unwrap(), result);
    }

    // Run the 8-thread parallel version
    {
        let runtime = Builder::new_multi_thread()
            .worker_threads(8)
            .build()
            .unwrap();

        let start = SystemTime::now();
        let result = runtime
            .block_on(runtime.spawn(fib_par_0(n)))
            .expect("Fibonacci calculation failed");

        println!("parallel: {:?} - {}", start.elapsed().unwrap(), result);
    }
}
