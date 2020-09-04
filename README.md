# joinable_threadpool
A light threadpool wrapper for making join-able threads similar to
[`std::thread::spawn`](https://doc.rust-lang.org/std/thread/fn.spawn.html),
but are run from a pool of threads managed by
[`threadpool::ThreadPool`](https://docs.rs/threadpool/1.8.1/threadpool/).

## Example
```rust
use joinable_threadpool::{ThreadPool, JoinHandle};

let pool = ThreadPool::new(4);

let handles: Vec<JoinHandle<String>> = (0..8)
    .map(|i| pool.spawn(move || format!("Hello from thread {}", i)))
    .collect();

for handle in handles {
    let msg: String = handle.join().unwrap();
    println!("Message: {}", msg);
}
```