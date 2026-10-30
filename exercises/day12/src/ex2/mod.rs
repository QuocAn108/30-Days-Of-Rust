pub mod math_operations;
pub mod statistics;

use rand::seq::SliceRandom;
use rand::Rng;

pub fn ex2() {
    println!("=== Exercise: Level 2 ===");

    // 1. Use unit_converter to convert temperatures (Celsius to Fahrenheit)
    let c = 25.0;
    let f = unit_converter::celsius_to_fahrenheit(c);
    println!("Temperature Converter: {:.1}°C = {:.1}°F", c, f);

    let f_boil = 212.0;
    println!(
        "Temperature Converter: {:.1}°F = {:.1}°C",
        f_boil,
        unit_converter::fahrenheit_to_celsius(f_boil)
    );

    // 2. Module statistics: calculate mean and median of numbers
    let scores = vec![85.0, 90.0, 78.0, 92.0, 88.0];
    if let Some(avg) = statistics::mean(&scores) {
        println!("Statistics: Mean score = {:.2}", avg);
    }
    if let Some(med) = statistics::median(&scores) {
        println!("Statistics: Median score = {:.2}", med);
    }

    // 3. Module math_operations: addition, subtraction, multiplication, division
    println!("Math: 20.0 + 4.0 = {}", math_operations::add(20.0, 4.0));
    println!("Math: 20.0 - 4.0 = {}", math_operations::subtract(20.0, 4.0));
    println!("Math: 20.0 * 4.0 = {}", math_operations::multiply(20.0, 4.0));
    if let Some(res) = math_operations::divide(20.0, 4.0) {
        println!("Math: 20.0 / 4.0 = {}", res);
    }

    // 4. Nested module trigonometry inside math_operations: sine and cosine
    use std::f64::consts::PI;
    let angle_deg = 45.0;
    let angle_rad = math_operations::trigonometry::to_radians(angle_deg);
    println!(
        "Trigonometry: sin(45°) = {:.4}",
        math_operations::trigonometry::sin(angle_rad)
    );
    println!(
        "Trigonometry: cos(45°) = {:.4}",
        math_operations::trigonometry::cos(angle_rad)
    );
    println!(
        "Trigonometry: sin(PI/2) = {:.4}",
        math_operations::trigonometry::sin(PI / 2.0)
    );
    println!(
        "Trigonometry: cos(PI) = {:.4}",
        math_operations::trigonometry::cos(PI)
    );

    // 5. Use rand crate to generate random numbers
    let mut rng = rand::thread_rng();
    let random_number: i32 = rng.gen_range(1..=100);
    let dice: u32 = rng.gen_range(1..=6);
    let random_float: f64 = rng.gen_range(0.0..1.0);
    let colors = ["Red", "Green", "Blue", "Yellow", "Orange"];
    let random_color = colors.choose(&mut rng).unwrap_or(&"None");

    println!("Random (1..=100): {}", random_number);
    println!("Random Dice roll: 🎲 {}", dice);
    println!("Random Float: {:.4}", random_float);
    println!("Random Color chosen: {}", random_color);
    println!();
}
