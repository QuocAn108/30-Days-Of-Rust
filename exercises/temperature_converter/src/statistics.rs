/// Calculate the mean (average) of a slice of numbers.
/// Returns None if the slice is empty.
pub fn mean(numbers: &[f64]) -> Option<f64> {
    if numbers.is_empty() {
        return None;
    }
    let sum: f64 = numbers.iter().sum();
    Some(sum / numbers.len() as f64)
}

/// Calculate the median of a slice of numbers.
/// Returns None if the slice is empty.
pub fn median(numbers: &[f64]) -> Option<f64> {
    if numbers.is_empty() {
        return None;
    }
    let mut sorted = numbers.to_vec();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    let len = sorted.len();
    let mid = len / 2;
    if len % 2 == 0 {
        Some((sorted[mid - 1] + sorted[mid]) / 2.0)
    } else {
        Some(sorted[mid])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mean() {
        let nums = [1.0, 2.0, 3.0, 4.0, 5.0];
        assert_eq!(mean(&nums), Some(3.0));
        assert_eq!(mean(&[]), None);
    }

    #[test]
    fn test_median_odd() {
        let nums = [5.0, 1.0, 3.0];
        assert_eq!(median(&nums), Some(3.0));
    }

    #[test]
    fn test_median_even() {
        let nums = [1.0, 2.0, 3.0, 4.0];
        assert_eq!(median(&nums), Some(2.5));
    }

    #[test]
    fn test_median_empty() {
        assert_eq!(median(&[]), None);
    }
}

