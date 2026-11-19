use day13::subtract;

#[test]
fn test_subtraction_integration_basic() {
    assert_eq!(subtract(10, 4), 6);
    assert_eq!(subtract(100, 45), 55);
}

#[test]
fn test_subtraction_integration_negative_numbers() {
    assert_eq!(subtract(-10, -5), -5);
    assert_eq!(subtract(-10, 5), -15);
    assert_eq!(subtract(10, -5), 15);
}

#[test]
fn test_subtraction_integration_zero() {
    assert_eq!(subtract(0, 0), 0);
    assert_eq!(subtract(50, 0), 50);
    assert_eq!(subtract(0, 50), -50);
}
