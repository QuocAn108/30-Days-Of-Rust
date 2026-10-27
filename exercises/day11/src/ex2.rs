use std::f64::consts::PI;

trait Area<T> {
    fn area(&self) -> T;
}

struct Rectangle<T> {
    height: T,
    width: T,
}
struct Circle<T> {
    radius: T,
}

impl Area<f64> for Rectangle<f64> {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}
impl Area<f64> for Circle<f64> {
    fn area(&self) -> f64 {
        (self.radius * self.radius) * PI
    }
}

fn print_area<T, S: Area<T>>(shape: &S)
where
    T: std::fmt::Display,
{
    println!("Area from generic function: {}", shape.area());
}

trait Speak {
    fn speak(&self);
}

struct Dog;
struct Cat;

impl Speak for Dog {
    fn speak(&self) {
        println!("Woof!");
    }
}
impl Speak for Cat {
    fn speak(&self) {
        println!("Meow!");
    }
}

trait Move {
    fn move_to(&self);
}

struct Car;
struct Person;

impl Move for Car {
    fn move_to(&self) {
        println!("Car is driving...");
    }
}
impl Move for Person {
    fn move_to(&self) {
        println!("Person is walking...");
    }
}

pub fn ex2() {
    let rect = Rectangle {
        width: 5.5,
        height: 2.0,
    };
    let circle = Circle { radius: 3.0 };
    print_area(&rect);
    print_area(&circle);

    let animals: [&dyn Speak; 2] = [&Dog, &Cat];
    for animal in animals.iter() {
        animal.speak();
    }

    let movers: Vec<Box<dyn Move>> = vec![Box::new(Car), Box::new(Person)];
    for mover in movers {
        mover.move_to();
    }
}
