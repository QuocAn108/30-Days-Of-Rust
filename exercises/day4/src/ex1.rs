pub fn len_str(string: &str) -> usize {
    string.len()
}

pub fn max(a: i32, b: i32) -> i32 {
    if a > b { a } else { b }
}

pub fn convert_celsius_to_fahrenheit(celsius: f64) -> f64 {
    (celsius * 9.0 / 5.0) + 32.0
}

pub fn multiply_number(num: i32) {
    for i in 1..=10 {
        println!("{} x {} = {}", num, i, num * i);
    }
}

pub fn gcd_2num(num1: i32, num2: i32) -> i32 {
    let mut a = num1.abs();
    let mut b = num2.abs();
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}
