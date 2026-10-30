mod math_operations {
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    pub fn subtract(a: i32, b: i32) -> i32 {
        if a > b {
            a - b
        } else {
            b - a
        }
    }
}

pub fn ex() {
    println!("=== Hands-On Challenge ===");
    let sum = math_operations::add(3, 3);
    let subtract = math_operations::subtract(2, 3);
    println!("sum: {}", sum);
    println!("subtract: {}", subtract);

    let area = geometry::rectangle_area(10.0, 5.0);
    println!("Area of rectangle: {}", area);
    println!();
}

