# Fibonacci LOL

A response to [PK's GoFib](https://github.com/pdk/gofib) parallel go fibonacci which is a response to
[this reddit thread](https://www.reddit.com/r/javascript/comments/lg80p2/nodejs_14_is_over_20x_faster_than_python38_for/gmq2s0b/). He was curious to see what a similar, concurrent version of a rust fibonacci number calculator would look like. This project reproduces the spirit of that request, so it doesn't do have any optimizations to try to get at the actual answer faster. 

This implementation explicitely performs both recursive executions at each step, so it is only a demonstration of exponential explosion in a bounded tokio runtime vs in a single native thread. Here are the results. Spoilers, doing this in tokio is not the right thing to do.
   
    cargo run --release 35
    normal  : Ok(31.106 ms)   - 102334155
    parallel: Ok(3.46844 s)   - 102334155 

    cargo run --release 40
    normal  : Ok(281.493 ms)  - 102334155
    parallel: Ok(39.145676 s) - 102334155
    
    cargo run --release 47
    normal  : Ok(6.48205 s)     - 2971215073
    parallel: Ok(1596.095517 s) - 2971215073
    
