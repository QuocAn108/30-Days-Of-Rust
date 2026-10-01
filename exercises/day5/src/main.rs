fn main() {
    let s = String::from("Hello, world!");
    let length = get_string(s.clone());
    println!("The length of the string is: {}", length);
    println!("{}", s);
    let mut a = String::from("Hello");
    println!("{}", a);
    modify_string(&mut a);
    println!("{}", a);

    let mut x = 5;
    let mut y = 10;
    println!("Before swap: x = {}, y = {}", x, y);
    swap(&mut x, &mut y);
    println!("After swap: x = {}, y = {}", x, y);

    let v = vec![1, 2, 3, 4, 5];
    let cloned_v = clone_vector(&v);
    println!("Original vector: {:?}", v);
    println!("Cloned vector: {:?}", cloned_v);

    let n = 5;
    let result = factorial(&n);
    println!("Factorial of {} is {}", n, result);

    let text = "hello world";
    let target_char = 'o';
    let count = count_char_occurrences(text, target_char);
    println!(
        "The character '{}' occurs {} times in the string \"{}\".",
        target_char, count, text
    );
}

fn get_string(s: String) -> String {
    s.len().to_string()
}

fn modify_string(s: &mut String) {
    s.push_str(", An Dep Trai!");
}

fn swap(a: &mut i32, b: &mut i32) {
    std::mem::swap(a, b);
}

fn clone_vector(v: &Vec<i32>) -> Vec<i32> {
    v.clone()
}

fn factorial(n: &u32) -> u32 {
    if *n == 0 {
        1
    } else {
        let prev = *n - 1;
        *n * factorial(&prev)
    }
}
fn count_char_occurrences(s: &str, target: char) -> usize {
    s.chars().filter(|&c| c == target).count()
}
