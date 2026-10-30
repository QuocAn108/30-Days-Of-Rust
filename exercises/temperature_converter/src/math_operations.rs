pub fn add(a: f64, b: f64) -> f64 {
    a + b
}

pub fn subtract(a: f64, b: f64) -> f64 {
    a - b
}

pub fn multiply(a: f64, b: f64) -> f64 {
    a * b
}

pub fn divide(a: f64, b: f64) -> Option<f64> {
    if b == 0.0 {
        None
    } else {
        Some(a / b)
    }
}

pub mod trigonometry {
    pub fn sin(angle_radians: f64) -> f64 {
        angle_radians.sin()
    }

    pub fn cos(angle_radians: f64) -> f64 {
        angle_radians.cos()
    }

    pub fn to_radians(degrees: f64) -> f64 {
        degrees.to_radians()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_math() {
        assert_eq!(add(10.0, 5.0), 15.0);
        assert_eq!(subtract(10.0, 5.0), 5.0);
        assert_eq!(multiply(10.0, 5.0), 50.0);
        assert_eq!(divide(10.0, 5.0), Some(2.0));
        assert_eq!(divide(10.0, 0.0), None);
    }

    #[test]
    fn test_trigonometry() {
        use std::f64::consts::PI;
        let zero = trigonometry::sin(0.0);
        assert!((zero - 0.0).abs() < 1e-6);

        let half_pi = trigonometry::sin(PI / 2.0);
        assert!((half_pi - 1.0).abs() < 1e-6);

        let cos_zero = trigonometry::cos(0.0);
        assert!((cos_zero - 1.0).abs() < 1e-6);
    }
}

