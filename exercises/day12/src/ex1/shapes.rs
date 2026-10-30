use std::f64::consts::PI;

pub fn circle_area(radius: f64) -> f64 {
    PI * radius * radius
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circle_area() {
        let area = circle_area(1.0);
        assert!((area - PI).abs() < 1e-6);

        let area_r2 = circle_area(2.0);
        assert!((area_r2 - (4.0 * PI)).abs() < 1e-6);
    }
}
