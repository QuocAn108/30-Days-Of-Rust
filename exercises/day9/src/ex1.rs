use regex::Regex;
use std::io;
use std::io::Write;

pub fn division_by_zero(number: i32) -> Result<i32, io::Error> {
    if number % 2 == 0 {
        Ok(number / 2)
    } else {
        Err(io::Error::new(io::ErrorKind::Other, "Division by zero"))
    }
}

pub fn get_number(list: Vec<i32>, index: usize) -> Option<i32> {
    if index < list.len() {
        Some(list[index])
    } else {
        None
    }
}
pub fn validate_input_regex(input: &str) -> Result<bool, regex::Error> {
    let re = Regex::new(r"^[a-zA-Z0-9 ]+$").unwrap();
    Ok(re.is_match(input))
}

pub fn ex1() {
    match division_by_zero(5) {
        Ok(result) => println!("Result: {}", result),
        Err(e) => eprintln!("Error: {}", e),
    }

    match get_number(vec![1, 2, 3], 5) {
        Some(number) => println!("Number at index 5: {}", number),
        None => eprintln!("Index out of bounds"),
    }

    println!("User input value: ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let input = input.trim();
    match validate_input_regex(input) {
        Ok(valid) => {
            if valid {
                println!("{} is a valid input", input);
                println!("Press Enter to exit...");
            } else {
                println!("Input is invalid");
                io::stdout().flush().unwrap();
                println!("Re-enter input value: ");
                io::stdin().read_line(&mut String::new()).unwrap();
            }
        }
        Err(e) => eprintln!("Error validating input: {}", e),
    }
}
