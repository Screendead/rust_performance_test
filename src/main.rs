pub mod threaded;

use crate::threaded::Threaded;

fn main() {
    let _threaded = Threaded::new(1_000_000_000);

    _threaded.run(100_000_000, Threaded::thread_result_squares);
}
