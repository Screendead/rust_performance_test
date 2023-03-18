use std::thread;
use std::time::Instant;

fn main() {
    let max_threads = max_threads();
    let mut threads = vec![];

    let start_time = Instant::now();

    for i in 1..=max_threads {
        let thread_start_time = Instant::now();
        let thread = thread::spawn(move || {
            let result: i64 = thread_result(100_000_000);

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

fn max_threads() -> usize {
    num_cpus::get()
}

fn thread_result(max: usize) -> i64 {
    let mut result: i64 = 0;

    for i in 1..=max {
        result += i as i64;
    }

    result
}

#[test]
fn test_max_threads() {
    let expected = num_cpus::get();
    let actual = max_threads();
    assert_eq!(expected, actual);
}

#[test]
fn test_thread_result() {
    let expected = 5000050000;
    let start_time = Instant::now();
    let actual = thread_result(100_000);
    let elapsed = start_time.elapsed().as_millis();
    assert_eq!(expected, actual);
    println!("Thread result test passed in {} ms", elapsed);
}
