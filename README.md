# Fibonacci LOL

A response to [PK's GoFib](https://github.com/pdk/gofib) parallel go fibonacci which is a response to
[this reddit thread](https://www.reddit.com/r/javascript/comments/lg80p2/nodejs_14_is_over_20x_faster_than_python38_for/gmq2s0b/). He was curious to see what a similar, concurrent version of a rust fibonacci number calculator would look like. This project reproduces the spirit of that request, so it doesn't do have any optimizations to try to get at the actual answer faster.

This implementation explicitly performs both recursive executions at each step, so it is only a demonstration of exponential explosion in a bounded tokio runtime vs in a single native thread. Parallelism is achieved by spawning tokio tasks for the recursive steps in the first levels which yields 2^3 = 8 threads to perform the standard fibonacci calculations in parallel. Here are the results.

    cargo run --release 35
    normal  : 14.345ms - 9227465
    parallel: 4.192ms  - 9227465

    cargo run --release 40
    normal  : 154.561ms - 102334155
    parallel: 43.028ms  - 102334155

    cargo run --release 47
    normal  : 4.479127s - 2971215073
    parallel: 1.156553s - 2971215073

    cargo run --release 55
    normal  : 80.618475s - 53316291173
    parallel: 21.005999s - 53316291173

