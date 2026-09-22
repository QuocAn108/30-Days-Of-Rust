// fn even_or_odd(num: i32) -> &'static str {
//     if num % 2 == 0 { "Even" } else { "Odd" }
// }
//
// fn area_circle(radius: f64) -> f64 {
//     std::f64::consts::PI * radius * radius
// }
//
// fn greatest_common_divisor(a: i32, b: i32) -> i32 {
//     let mut a = a.abs();
//     let mut b = b.abs();
//     while b != 0 {
//         let temp = b;
//         b = a % b;
//         a = temp;
//     }
//     a
// }
//
// fn main() {
//     println!("{}", even_or_odd(5));
//     println!("{:.2}", area_circle(3.0));
//     println!("{}", greatest_common_divisor(48, 18));
// }

mod ex1;
mod ex2;

fn main() {
    let string = "Hello, world!";
    println!("Length of '{}': {}", string, ex1::len_str(string));

    let a = 10;
    let b = 20;
    println!("Max of {} and {}: {}", a, b, ex1::max(a, b));

    let celsius = 25.0;
    println!(
        "{}°C in Fahrenheit: {:.2}°F",
        celsius,
        ex1::convert_celsius_to_fahrenheit(celsius)
    );

    let num = 5;
    println!("Multiplication table for {}:", num);
    ex1::multiply_number(num);

    let num1 = 48;
    let num2 = 18;
    println!(
        "GCD of {} and {}: {}",
        num1,
        num2,
        ex1::gcd_2num(num1, num2)
    );

    let n = 5;
    println!("Factorial of {}: {}", n, ex2::recursive_factorial(n));

    let numbers = [1.0, 2.0, 3.0, 4.0, 5.0];
    println!("Average of {:?}: {:.2}", numbers, ex2::avg_list(&numbers));

    let test_string = "Hello, World!";
    println!(
        "Number of vowels in '{}': {}",
        test_string,
        ex2::vowels_count(test_string)
    );

    let num1 = 10.0;
    let num2 = 5.0;
    let operator = '+';
    match ex2::operation_calculator(num1, num2, operator) {
        Some(result) => println!("{} {} {} = {}", num1, operator, num2, result),
        None => println!("Invalid operation or division by zero"),
    }
    let mut numbers_to_sort = [64, 34, 25, 12, 22, 11, 90];
    ex2::sort_bubble(&mut numbers_to_sort);
    println!("Sorted array: {:?}", numbers_to_sort);
    println!(
        "Area of circle with radius 3.0: {:.2}",
        ex2::area_circle(3.0)
    );
}
