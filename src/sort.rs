pub fn sort(max: i64) -> Vec<i64> {
    let base: i64 = 2; // an explicit type is required
    let exp = 32;

    if max >= base.pow(exp) {
        panic!("Max value is too large");
    }

    let mut result: Vec<i64> = (1..=max).collect();
    result.sort();
    result
}

#[test]
fn test_sort() {
    let expected: Vec<i64> = (1..=100_000).collect();
    let actual: Vec<i64> = sort(100_000);
    assert_eq!(expected, actual);
}

#[test]
#[should_panic]
fn test_sort_large() {
    let _result: Vec<i64> = sort(10_000_000_000);
}
