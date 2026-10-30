pub mod length {
    /// Convert meters to kilometers
    pub fn meters_to_kilometers(meters: f64) -> f64 {
        meters / 1000.0
    }

    /// Convert kilometers to meters
    pub fn kilometers_to_meters(kilometers: f64) -> f64 {
        kilometers * 1000.0
    }
}

pub mod temperature {
    /// Convert Celsius to Fahrenheit: (C * 9/5) + 32
    pub fn celsius_to_fahrenheit(celsius: f64) -> f64 {
        (celsius * 9.0 / 5.0) + 32.0
    }

    /// Convert Fahrenheit to Celsius: (F - 32) * 5/9
    pub fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
        (fahrenheit - 32.0) * 5.0 / 9.0
    }
}

// Re-export for direct top-level access
pub use length::{kilometers_to_meters, meters_to_kilometers};
pub use temperature::{celsius_to_fahrenheit, fahrenheit_to_celsius};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_meters_to_kilometers() {
        assert_eq!(meters_to_kilometers(1000.0), 1.0);
        assert_eq!(meters_to_kilometers(2500.0), 2.5);
        assert_eq!(meters_to_kilometers(0.0), 0.0);
    }

    #[test]
    fn test_kilometers_to_meters() {
        assert_eq!(kilometers_to_meters(1.0), 1000.0);
        assert_eq!(kilometers_to_meters(2.5), 2500.0);
    }

    #[test]
    fn test_celsius_to_fahrenheit() {
        assert_eq!(celsius_to_fahrenheit(0.0), 32.0);
        assert_eq!(celsius_to_fahrenheit(100.0), 212.0);
        assert_eq!(celsius_to_fahrenheit(-40.0), -40.0);
    }

    #[test]
    fn test_fahrenheit_to_celsius() {
        assert_eq!(fahrenheit_to_celsius(32.0), 0.0);
        assert_eq!(fahrenheit_to_celsius(212.0), 100.0);
        assert_eq!(fahrenheit_to_celsius(-40.0), -40.0);
    }
}
