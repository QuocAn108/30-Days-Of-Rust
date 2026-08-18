use std::io;
use std::io::Write;
use std::str::FromStr;

fn read_input<T: FromStr>(prompt: &str) -> T {
    print!("{}", prompt);
    io::stdout().flush().expect("Failed to flush stdout.");

    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line.");

    line.trim()
        .parse()
        .unwrap_or_else(|_| panic!("Failed to parse input."))
}


fn main() {
    /* 
     * This is an example of how to use the input function we defined above.
     * The only tricky bit is that we now need to explicitly type the variables.
     * This lets the parsing function (above) know what to expect.
     *
     * The two types we are using are:
     *  - i32 : a 32 bit integer 
     *  - String : a string of text
     *
     * Make it your own!
     *
     */ 
    
    let name: String = read_input("What is your name? ");
    let x: i32 = read_input("Enter a number: ");

    println!("{} input {}!", name, x);

}
