use day13::multiply;

#[test]
fn test_multiplication_integration_basic() {
    assert_eq!(multiply(6, 7), 42);
    assert_eq!(multiply(12, 12), 144);
}

#[test]
fn test_multiplication_integration_signs() {
    assert_eq!(multiply(-6, 7), -42);
    assert_eq!(multiply(6, -7), -42);
    assert_eq!(multiply(-6, -7), 42);
}

#[test]
fn test_multiplication_integration_zero_and_identity() {
    assert_eq!(multiply(0, 999), 0);
    assert_eq!(multiply(999, 0), 0);
    assert_eq!(multiply(1, 999), 999);
    assert_eq!(multiply(-1, 999), -999);
}
