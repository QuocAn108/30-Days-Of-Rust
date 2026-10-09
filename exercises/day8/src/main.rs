fn main() {
    let mut greeting = String::from("Hello");

    greeting.push_str(", world!");
    println!("{}", greeting);

    let substring = &greeting[0..5]; // "Hello"
    println!("Substring: {}", substring);
}
