mod math_operations;
mod statistics;

use unit_converter::temperature::{celsius_to_fahrenheit, fahrenheit_to_celsius};

fn main() {
    println!("=== Temperature Converter Binary Crate ===");

    // 1. Temperature conversion using unit_converter crate
    let celsius = 25.0;
    let fahrenheit = celsius_to_fahrenheit(celsius);
    println!("{:.1}°C = {:.1}°F", celsius, fahrenheit);

    let boiling_f = 212.0;
    println!(
        "{:.1}°F = {:.1}°C",
        boiling_f,
        fahrenheit_to_celsius(boiling_f)
    );

    // 2. Statistics module usage
    let temperatures = vec![23.5, 25.0, 22.0, 28.0, 26.5];
    if let Some(avg) = statistics::mean(&temperatures) {
        println!("Average Temperature: {:.2}°C", avg);
    }
    if let Some(med) = statistics::median(&temperatures) {
        println!("Median Temperature: {:.2}°C", med);
    }

    // 3. Math operations module usage
    println!("Math: 12.5 + 7.5 = {}", math_operations::add(12.5, 7.5));
    println!("Math: 12.5 - 7.5 = {}", math_operations::subtract(12.5, 7.5));
    println!("Math: 12.5 * 2.0 = {}", math_operations::multiply(12.5, 2.0));
    match math_operations::divide(12.5, 2.5) {
        Some(res) => println!("Math: 12.5 / 2.5 = {}", res),
        None => println!("Math: Division by zero!"),
    }

    // 4. Nested trigonometry module usage
    use std::f64::consts::PI;
    let angle_deg = 30.0;
    let angle_rad = math_operations::trigonometry::to_radians(angle_deg);
    println!(
        "Trigonometry: sin(30°) = {:.4}",
        math_operations::trigonometry::sin(angle_rad)
    );
    println!(
        "Trigonometry: cos(30°) = {:.4}",
        math_operations::trigonometry::cos(angle_rad)
    );
    println!(
        "Trigonometry: sin(PI/2) = {:.4}",
        math_operations::trigonometry::sin(PI / 2.0)
    );
}
