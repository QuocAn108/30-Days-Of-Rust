trait Describe {
    fn describe(&self) -> String;
}

struct Car {
    brand: String,
}

impl Describe for Car {
    fn describe(&self) -> String {
        format!("This is a car from {}", self.brand)
    }
}

struct Bike {
    brand: String,
}

impl Describe for Bike {
    fn describe(&self) -> String {
        format!("This is a bike from {}", self.brand)
    }
}

pub fn ex() {
    let car = Car {
        brand: String::from("Toyota"),
    };
    let bike = Bike {
        brand: String::from("Yamaha"),
    };

    println!("{}", car.describe());
    println!("{}", bike.describe());
}
