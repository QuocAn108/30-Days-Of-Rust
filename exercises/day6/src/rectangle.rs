pub struct Rectangle {
    pub width: f64,
    pub length: f64,
}

impl Rectangle {
    pub fn cal_area(&self) -> f64 {
        self.width * self.length
    }

    pub fn cal_perimeter(&self) -> f64 {
        (self.width + self.length) / 2.0
    }
}
