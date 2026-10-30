pub fn rectangle_area(length: f64, width: f64) -> f64 {
    length * width
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = rectangle_area(2.0, 2.0);
        assert_eq!(result, 4.0);
    }
}
