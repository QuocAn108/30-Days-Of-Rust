// fn main() {
//     let name = "Ryan Hoang";
//     let mut age = 21;
//     age += 1;
//     let favorite_number = 10;
//     let is_learning_rust = true;
//     let initial = 'Y';
//
//     println!("Name: {}", name);
//     println!("Updated Age: {}", age);
//     println!("Favorite Number: {}", favorite_number);
//     println!("Learnin rust: {}", is_learning_rust);
//     println!("Initital: {}", initial);
// }

fn main() {
    let my_age = 22;
    println!("My age is: {}", my_age);
    let mut my_height = 175;
    my_height += 5;
    println!("My height is: {} cm", my_height);
    let my_name = "Ryan Hoang";
    println!("My name is: {}", my_name);
    let is_student = true;
    if is_student {
        println!("I am a student.");
    } else {
        println!("I am not a student.");
    }
    let birth_year = 2026 - my_age;
    println!("I was born in: {}", birth_year);

    let my_integer: i32 = 10;
    let my_float: f64 = 3.14;
    let is_learning_rust: bool = true;
    let favorite_letter: char = 'A';
    let my_scores: [i32; 5] = [10, 10, 10, 10, 10];
    let hobbys: [&str; 3] = ["Sporting", "Coding", "Gaming"];
    println!("My integer: {}", my_integer);
    println!("My float: {}", my_float);
    println!("Is learning Rust: {}", is_learning_rust);
    println!("My favorite character: {}", favorite_letter);
    for score in my_scores.iter() {
        println!("My score: {}", score);
    }
    for hobby in hobbys.iter() {
        println!("I enjoy {}!", hobby);
    }
}
