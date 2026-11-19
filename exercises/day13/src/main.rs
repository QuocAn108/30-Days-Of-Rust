use day13::{multiply, subtract};

fn main() {
    println!("🦀 Day 13: Testing in Rust 🚀");

    let x = 20;
    let y = 6;

    println!("Subtract: {} - {} = {}", x, y, subtract(x, y));
    println!("Multiply: {} * {} = {}", x, y, multiply(x, y));
}
