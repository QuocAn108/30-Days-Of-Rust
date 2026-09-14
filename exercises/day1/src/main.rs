fn main() {
    println!("Hello, World!");
    let x = 5;
    let mut x = 10;
    println!("The value of x is: {}", x);

    let integer_num: i32 = -42;
    let float_num: f64 = 15.0;
    let is_rust_fun: bool = true;
    let text_string: &str = "Học Rust mỗi ngày";

    println!(
        "Kiểu dữ liệu: Int={}, Float={}, Bool={}, String=\"{}\"",
        integer_num, float_num, is_rust_fun, text_string
    );

    if x > 5 {
        println!("x lớn hơn 5");
    } else {
        println!("x nhỏ hơn hoặc bằng 5");
    }

    println!("Vòng lặp while:");
    let mut counter = 0;
    while counter < 3 {
        print!("{} ", counter);
        counter += 1;
    }
    println!();

    println!("Vòng lặp for:");
    for i in 1..=3 {
        print!("{} ", i);
    }
    println!();
}
