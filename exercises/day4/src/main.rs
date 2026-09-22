fn even_or_odd(num: i32) -> &'static str {
    if num % 2 == 0 { "Even" } else { "Odd" }
}

fn area_circle(radius: f64) -> f64 {
    std::f64::consts::PI * radius * radius
}

fn greatest_common_divisor(a: i32, b: i32) -> i32 {
    let mut a = a.abs();
    let mut b = b.abs();
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

fn main() {
    println!("{}", even_or_odd(5));
    println!("{}", area_circle(3.0));
    println!("{}", greatest_common_divisor(48, 18));
}
