#[allow(dead_code)]
pub fn squares(max: i64) -> i128 {
    let base: i64 = 2; // an explicit type is required
    let exp = 32;

    if max >= base.pow(exp) {
        panic!("Max value is too large");
    }

    (1..=max).fold(0, |acc, x| acc + x as i128)
}

#[test]
fn test_squares() {
    let expected: i128 = 5000050000;
    let actual: i128 = squares(100_000);
    assert_eq!(expected, actual);
}

#[test]
#[should_panic]
fn test_squares_large() {
    let _result: i128 = squares(10_000_000_000);
}
