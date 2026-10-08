pub enum Card {
    CreditCard(String),
    DebitCard(String),
    Cash,
    Paypal,
}

pub fn print_card(card: Card) {
    match card {
        Card::CreditCard(number) => println!("Paid with Credit Card: {}", number),
        Card::DebitCard(number) => println!("Paid with Debit Card: {}", number),
        Card::Cash => println!("Paid with Cash"),
        Card::Paypal => println!("Paid with Paypal"),
    }
}

pub enum Weather {
    Sunny,
    Rainy,
    Cloudy,
    Windy,
}

pub fn print_weather(weather: Weather) {
    match weather {
        Weather::Sunny => println!("The weather is sunny!"),
        Weather::Rainy => println!("It's raining outside."),
        Weather::Cloudy => println!("The sky is cloudy."),
        Weather::Windy => println!("It's quite windy today."),
        _ => println!("Unknown weather condition."),
    }
}

pub enum Device {
    Laptop(String),
    Smartphone(String),
    Tablet(String),
}

pub fn print_device(device: Device) {
    match device {
        Device::Laptop(model) => println!("Device is a Laptop: {}", model),
        Device::Smartphone(model) => println!("Device is a Smartphone: {}", model),
        Device::Tablet(model) => println!("Device is a Tablet: {}", model),
    }
}

pub enum trafficLight {
    Red(String),
    Yellow(String),
    Green(String),
}

pub fn print_traffic_light(light: trafficLight) {
    match light {
        trafficLight::Red(time) => println!("Red: {}", time),
        trafficLight::Yellow(time) => println!("Yellow: {}", time),
        trafficLight::Green(time) => println!("Green: {}", time),
    }
}

pub enum FileFormat {
    PDF(i32),
    Word(i32),
    Excel(i32),
}

pub fn print_format_size(format: FileFormat) {
    match format {
        FileFormat::PDF(size) => println!("PDF file size: {} KB", size),
        FileFormat::Word(size) => println!("Word file size: {} KB", size),
        FileFormat::Excel(size) => println!("Excel file size: {} KB", size),
    }
}

pub enum Status {
    Active,
    Inactive,
    Suspended,
}

pub fn check_status(status: Status) {
    if let Status::Active = status {
        println!("Status is Active");
    } else if let Status::Inactive = status {
        println!("Status is Inactive");
    } else if let Status::Suspended = status {
        println!("Status is Suspended");
    }
}

pub enum OrderStatus {
    Pending,
    Shipped,
    Delivered,
}

pub fn check_order_status(status: OrderStatus) {
    match status {
        OrderStatus::Pending => println!("Order is Pending"),
        OrderStatus::Shipped => println!("Order has been Shipped"),
        OrderStatus::Delivered => println!("Order has been Delivered"),
    }
}

pub enum shape {
    Circle(f64),
    Rectangle(f64, f64),
    Triangle(f64, f64, f64),
}
use std::f64::consts::PI;
pub fn calculate_area(shape: shape) {
    let pi = PI;
    match shape {
        shape::Circle(radius) => println!("Area of Circle: {}", pi * radius * radius),
        shape::Rectangle(width, height) => println!("Area of Rectangle: {}", width * height),
        shape::Triangle(base, height, _) => println!("Area of Triangle: {}", 0.5 * base * height),
    }
}
