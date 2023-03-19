pub fn max_threads() -> usize {
    num_cpus::get()
}

#[test]
fn test_max_threads() {
    let expected: usize = num_cpus::get();
    let actual: usize = max_threads();
    assert_eq!(expected, actual);
}
