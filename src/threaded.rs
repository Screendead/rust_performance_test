use std::{
    thread::{self, JoinHandle},
    time::{Duration, Instant},
};

pub struct Threaded {
    pub threads: usize,
    pub chunk_size: usize,
}

impl Threaded {
    pub fn new(chunk_size: usize) -> Self {
        Self {
            threads: Self::max_threads(),
            chunk_size,
        }
    }

    pub fn max_threads() -> usize {
        num_cpus::get()
    }

    pub fn run(&self, max: i64, func: fn(i64) -> i128) {
        let mut threads: Vec<JoinHandle<()>> = vec![];
        let start_time: Instant = Instant::now();

        for i in 1..=self.threads {
            let thread_start_time: Instant = Instant::now();
            let thread: JoinHandle<()> = thread::spawn(move || {
                let result: i128 = func(max);

                let thread_elapsed: Duration = thread_start_time.elapsed();
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

        let total_elapsed: Duration = start_time.elapsed();
        println!("Total elapsed time: {} ms", total_elapsed.as_millis());
    }

    pub fn thread_result_squares(max: i64) -> i128 {
        let base: i64 = 2; // an explicit type is required
        let exp = 32;

        if max >= base.pow(exp) {
            panic!("Max value is too large");
        }

        (1..=max).fold(0, |acc, x| acc + x as i128)
    }

    pub fn thread_result_sort(max: i64) -> Vec<i64> {
        let base: i64 = 2; // an explicit type is required
        let exp = 32;

        if max >= base.pow(exp) {
            panic!("Max value is too large");
        }

        let mut result: Vec<i64> = (1..=max).collect();
        result.sort();
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_threaded() {
        let _threaded = Threaded::new(1_000_000_000);

        _threaded.run(100_000, Threaded::thread_result_squares);
    }

    #[test]
    fn test_max_threads() {
        let expected: usize = num_cpus::get();
        let actual: usize = Threaded::max_threads();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_thread_result_squares() {
        let expected: i128 = 5000050000;
        let start_time: Instant = Instant::now();
        let actual: i128 = Threaded::thread_result_squares(100_000);
        let elapsed: u128 = start_time.elapsed().as_millis();
        assert_eq!(expected, actual);
        println!("Thread result test passed in {} ms", elapsed);
    }

    #[test]
    #[should_panic]
    fn test_thread_result_squares_large() {
        let _result: i128 = Threaded::thread_result_squares(10_000_000_000);
    }

    #[test]
    fn test_thread_result_sort() {
        let expected: Vec<i64> = (1..=100_000).collect();
        let start_time: Instant = Instant::now();
        let actual: Vec<i64> = Threaded::thread_result_sort(100_000);
        let elapsed: u128 = start_time.elapsed().as_millis();
        assert_eq!(expected, actual);
        println!("Thread result test passed in {} ms", elapsed);
    }

    #[test]
    #[should_panic]
    fn test_thread_result_sort_large() {
        let _result: Vec<i64> = Threaded::thread_result_sort(10_000_000_000);
    }
}
