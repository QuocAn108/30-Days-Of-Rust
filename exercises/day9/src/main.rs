mod ex;
mod ex1;
mod ex2;
fn main() {
    match ex::read_file("src/test.txt") {
        Ok(contents) => println!("File contents: {}", contents),
        Err(e) => eprintln!("Error reading file: {}", e),
    }
    ex1::ex1();
    ex2::ex2();
}
