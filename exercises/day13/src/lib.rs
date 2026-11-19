//! # Day 13: Testing in Rust
//!
//! Module providing basic arithmetic operations (`subtract`, `multiply`)
//! with unit tests, integration tests, and documentation tests.

/// Subtracts the second number from the first (`a - b`).
///
/// # Examples
///
/// ```
/// use day13::subtract;
///
/// let result = subtract(5, 2);
/// assert_eq!(result, 3);
///
/// assert_eq!(subtract(10, 5), 5);
/// assert_eq!(subtract(2, 7), -5);
/// ```
pub fn subtract(a: i32, b: i32) -> i32 {
    a - b
}

/// Multiplies two integers (`a * b`).
///
/// # Examples
///
/// ```
/// use day13::multiply;
///
/// let result = multiply(4, 5);
/// assert_eq!(result, 20);
///
/// assert_eq!(multiply(-3, 6), -18);
/// assert_eq!(multiply(-4, -5), 20);
/// assert_eq!(multiply(0, 100), 0);
/// ```
pub fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_subtract() {
        // Basic subtraction
        assert_eq!(subtract(5, 2), 3);
        assert_eq!(subtract(10, 5), 5);

        // Subtraction with negative numbers
        assert_eq!(subtract(-5, -2), -3);
        assert_eq!(subtract(-5, 2), -7);
        assert_eq!(subtract(5, -2), 7);

        // Subtraction with zero
        assert_eq!(subtract(5, 0), 5);
        assert_eq!(subtract(0, 5), -5);
        assert_eq!(subtract(0, 0), 0);

        // Subtracting identical numbers
        assert_eq!(subtract(100, 100), 0);
    }

    #[test]
    fn test_multiply() {
        // Positive integers
        assert_eq!(multiply(2, 3), 6);
        assert_eq!(multiply(4, 5), 20);

        // Negative and positive integers
        assert_eq!(multiply(-2, 3), -6);
        assert_eq!(multiply(2, -3), -6);

        // Two negative integers
        assert_eq!(multiply(-4, -5), 20);

        // Multiplication by zero
        assert_eq!(multiply(0, 10), 0);
        assert_eq!(multiply(10, 0), 0);
        assert_eq!(multiply(0, 0), 0);

        // Multiplication by one (identity property)
        assert_eq!(multiply(7, 1), 7);
        assert_eq!(multiply(1, 7), 7);
        assert_eq!(multiply(-7, 1), -7);
    }
}
