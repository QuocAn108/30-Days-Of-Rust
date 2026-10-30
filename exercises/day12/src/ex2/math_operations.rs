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

