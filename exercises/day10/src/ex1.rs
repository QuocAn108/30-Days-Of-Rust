fn min_value<T: PartialOrd>(list: &[T]) -> &T {
    let mut min = list.first().expect("List cannot be empty");
    for i in list {
        if i < min {
            min = i;
        }
    }
    min
}
use std::fmt::Display;
use std::ops::Mul;

struct Rectangle<T> {
    width: T,
    length: T,
}

impl<T> Rectangle<T>
where
    T: Mul<Output = T> + Copy + Display,
{
    fn area(&self) {
        let area = self.width * self.length;
        println!("Area of Rectangle: {}", area);
    }
}

fn print_element<T: Display>(list: Vec<T>) {
    for i in list {
        print!("{} ", i);
    }
    println!();
}

pub fn ex1() {
    let number = vec![10, 20, 43, 12, 2, 32];
    // let number: Vec<i32> = Vec::new();

    println!("Min value of list number: {}", min_value(&number));

    let int_rect = Rectangle {
        width: 10,
        length: 20,
    };
    int_rect.area();
    let float_rect = Rectangle {
        width: 5.5,
        length: 2.0,
    };
    float_rect.area();

    let numbers = vec![1, 2, 3, 4, 5];

    print_element(numbers);

    let words = vec!["Rust", "Generics", "Type"];
    print_element(words);
}
