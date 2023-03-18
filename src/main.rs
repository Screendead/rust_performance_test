use std::thread;
use std::time::Instant;

fn main() {
    let max_threads = num_cpus::get();
    let mut threads = vec![];

    let start_time = Instant::now();

    for i in 1..=max_threads {
        let thread_start_time = Instant::now();
        let thread = thread::spawn(move || {
            let mut result: i128 = 0;

            for j in 1..=10_000_000_000 {
                result += j;
            }

            let thread_elapsed = thread_start_time.elapsed();
            println!(
                "Thread {}: Result = {} ({} ms)",
                i,
                result,
                thread_elapsed.as_millis()
            );
        });

        threads.push(thread);
    }

    for thread in threads {
        thread.join().unwrap();
    }

    let total_elapsed = start_time.elapsed();
    println!("Total elapsed time: {} ms", total_elapsed.as_millis());
}
