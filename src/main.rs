mod max_threads;
mod sort;
mod squares;

use crate::max_threads::max_threads;
use crate::sort::sort;
use std::{
    thread::{self, JoinHandle},
    time::{Duration, Instant},
};

fn main() {
    let func = sort;
    let max: i64 = 1_000_000_000;

    let mut threads: Vec<JoinHandle<()>> = vec![];
    let start_time: Instant = Instant::now();

    for i in 1..=max_threads() {
        let thread_start_time: Instant = Instant::now();
        let thread: JoinHandle<()> = thread::spawn(move || {
            let result: Vec<i64> = func(max);

            let thread_elapsed: Duration = thread_start_time.elapsed();
            println!(
                "Thread {}: Result = {} ({} ms)",
                i,
                result.len(),
                thread_elapsed.as_millis()
            );
        });

        threads.push(thread);
    }

    for thread in threads {
        thread.join().unwrap();
    }

    let total_elapsed: Duration = start_time.elapsed();
    println!("Total elapsed time: {} ms", total_elapsed.as_millis());
}
