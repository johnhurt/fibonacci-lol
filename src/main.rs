use tokio::runtime::Builder;
use tokio::spawn;

use std::env::args;
use std::time::SystemTime;

#[async_recursion::async_recursion]
async fn fib_par(n: u64) -> u64 {
    if n < 2 {
        return n;
    }

    let v1_future = spawn(fib_par(n - 2));
    let v2_future = spawn(fib_par(n - 1));

    v1_future.await.unwrap() + v2_future.await.unwrap()
}

fn fib(n: u64) -> u64 {
    if n < 2 {
        return n;
    }

    fib(n - 1) + fib(n - 2)
}

fn main() {
    let n = args()
        .skip(1) // skip the invocation
        .map(|n| n.parse::<u64>())
        .next()
        .expect("This program takes a single non-negative integer argument")
        .ok()
        .expect("Failed to parse first argument as an non-negative integer");

    {
        let start = SystemTime::now();
        let result = fib(n);

        println!("normal  : {:?} - {}", start.elapsed(), result);
    }

    {
        let runtime = Builder::new_multi_thread().build().unwrap();

        let start = SystemTime::now();
        let result = runtime
            .block_on(runtime.spawn(fib_par(n)))
            .expect("Fibonacci calculation failed");

        println!("parallel: {:?} - {}", start.elapsed(), result);
    }
}
