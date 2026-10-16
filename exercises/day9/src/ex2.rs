use std::fmt;
use std::io::{self, Write};
#[derive(Debug, PartialEq)]
pub enum DivisionError {
    DivisionByZero,
    NotANumber,
}

impl fmt::Display for DivisionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DivisionError::DivisionByZero => write!(f, "Cannot divide by zero"),
            DivisionError::NotANumber => write!(f, "Input must not be NaN"),
        }
    }
}

pub fn divide(a: f64, b: f64) -> Result<f64, DivisionError> {
    if a.is_nan() || b.is_nan() {
        return Err(DivisionError::NotANumber);
    }
    if b == 0.0 {
        return Err(DivisionError::DivisionByZero);
    }
    Ok(a / b)
}
#[derive(Debug, PartialEq)]
pub enum BookLibraryError {
    BookNotFound,
    InvalidFormat,
}

impl fmt::Display for BookLibraryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BookLibraryError::BookNotFound => write!(f, "Book was not found"),
            BookLibraryError::InvalidFormat => write!(f, "Invalid file or book format"),
        }
    }
}

pub fn find_book(id: u32, filename: &str) -> Result<String, BookLibraryError> {
    if !filename.ends_with(".json") {
        return Err(BookLibraryError::InvalidFormat);
    }
    if id != 1 {
        return Err(BookLibraryError::BookNotFound);
    }
    Ok("The Rust Programming Language".to_string())
}
pub fn retrieve_element(slice: &[i32], index: usize) -> Option<i32> {
    slice.get(index).copied()
}

pub fn read_integer_from_stdin() -> Result<i32, String> {
    print!("Enter an integer: ");
    io::stdout().flush().map_err(|e| e.to_string())?;

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .map_err(|e| e.to_string())?;

    input
        .trim()
        .parse::<i32>()
        .map_err(|_| format!("'{}' is not a valid integer", input.trim()))
}

pub fn ex2() {
    println!("--- 1 & 4. Division ---");
    match divide(10.0, 2.5) {
        Ok(res) => println!("10.0 / 2.5 = {}", res),
        Err(e) => eprintln!("Error: {}", e),
    }
    match divide(5.0, 0.0) {
        Ok(res) => println!("5.0 / 0.0 = {}", res),
        Err(e) => eprintln!("Error: {}", e),
    }

    println!("\n--- 2. Library ---");
    match find_book(1, "library.json") {
        Ok(book) => println!("Found: {}", book),
        Err(e) => eprintln!("Error: {}", e),
    }
    match find_book(10, "library.json") {
        Ok(book) => println!("Found: {}", book),
        Err(e) => eprintln!("Error: {}", e),
    }

    println!("\n--- 5. Retrieve Element ---");
    let numbers = vec![100, 200, 300];
    let idx = 1;
    match retrieve_element(&numbers, idx) {
        Some(val) => println!("Element at index {}: {}", idx, val),
        None => println!("Index {} is out of bounds", idx),
    }

    println!("\n--- 6. User Input ---");
    match read_integer_from_stdin() {
        Ok(num) => println!("You entered: {}", num),
        Err(err) => eprintln!("Failed: {}", err),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_division_success() {
        assert_eq!(divide(10.0, 2.0), Ok(5.0));
    }

    #[test]
    fn test_division_by_zero() {
        assert_eq!(divide(10.0, 0.0), Err(DivisionError::DivisionByZero));
    }

    #[test]
    fn test_library_errors() {
        assert_eq!(
            find_book(1, "data.txt"),
            Err(BookLibraryError::InvalidFormat)
        );
        assert_eq!(
            find_book(99, "data.json"),
            Err(BookLibraryError::BookNotFound)
        );
        assert_eq!(
            find_book(1, "data.json"),
            Ok("The Rust Programming Language".to_string())
        );
    }

    #[test]
    fn test_retrieve_element() {
        let numbers = vec![10, 20, 30];
        assert_eq!(retrieve_element(&numbers, 1), Some(20));
        assert_eq!(retrieve_element(&numbers, 5), None);
    }
}
