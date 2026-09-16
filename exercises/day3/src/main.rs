use std::io;

fn main() {
    println!("Input your number: ");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let number: i32 = input.trim().parse().expect("Please type a number!");
    if number % 2 == 0 {
        println!("The number {} is even.", number);
    } else {
        println!("The number {} is odd.", number);
    }
    for i in 1..6 {
        println!("number:{} ", i,);
    }
    println!("Print Days of the week: ");
    let mut day_input = String::new();
    io::stdin()
        .read_line(&mut day_input)
        .expect("Failed to read line");
    let days = day_input.trim();
    match days {
        "Monday" => println!("Start of the week."),
        "Tuesday" => println!("Second day of the week."),
        "Wednesday" => println!("Midweek day."),
        "Thursday" => println!("Almost the weekend."),
        "Friday" => println!("Weekend is coming!"),
        _ => println!("It's not a regular day."),
    }
}
