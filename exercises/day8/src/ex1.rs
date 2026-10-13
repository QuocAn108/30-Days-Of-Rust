use std::collections::HashMap;
pub fn ex1() {
    let mut digit: Vec<i32> = Vec::new();
    digit.push(1);
    digit.push(2);
    digit.push(3);
    digit.push(4);
    digit.pop();
    for i in &digit {
        println!("{}", i);
    }
    // for i in 0..digit.len() {
    //     println!("{}", digit[i]);
    // }
    append_strings(String::from("hello"), " world");

    let mut person: HashMap<String, i32> = HashMap::new();
    person.insert(String::from("Alice"), 30);
    person.insert(String::from("Bob"), 25);
    print_name_age(person);
    let mut fruits: Vec<&str> = Vec::new();
    fruits.push("mango");
    fruits.push("grape");
    fruits.push("strawberry");
    fruits.push("apple");
    print_fruits(fruits);
}

fn append_strings(mut strings: String, character: &str) {
    strings.push_str(character);
    println!("{}", strings);
}

fn print_name_age(person: HashMap<String, i32>) {
    for (name, age) in person {
        println!("{} is {} years old", name, age);
    }
}

fn print_fruits(mut fruit: Vec<&str>) {
    fruit.sort();
    for i in &fruit {
        println!("{}", i);
    }
}
