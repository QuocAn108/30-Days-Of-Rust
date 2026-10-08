use crate::my_enum::{self, Card};
pub fn print_ex1() {
    let my_card = Card::CreditCard(String::from("1234-5678-9012-3456"));
    my_enum::print_card(my_card);

    let my_debit_card = Card::DebitCard(String::from("9876-5432-1098-7654"));
    my_enum::print_card(my_debit_card);

    let my_cash = Card::Cash;
    my_enum::print_card(my_cash);

    let my_paypal = Card::Paypal;
    my_enum::print_card(my_paypal);

    let sunny_weather = my_enum::Weather::Sunny;
    my_enum::print_weather(sunny_weather);
    let rainy_weather = my_enum::Weather::Rainy;
    my_enum::print_weather(rainy_weather);
    let cloudy_weather = my_enum::Weather::Cloudy;
    my_enum::print_weather(cloudy_weather);
    let windy_weather = my_enum::Weather::Windy;
    my_enum::print_weather(windy_weather);

    let macbook = my_enum::Device::Laptop(String::from("MacBook Pro"));
    my_enum::print_device(macbook);
    let ipad = my_enum::Device::Tablet(String::from("iPad Air"));
    my_enum::print_device(ipad);
    let iphone = my_enum::Device::Smartphone(String::from("iPhone 13"));
    my_enum::print_device(iphone);

    let red_light = my_enum::trafficLight::Red(String::from("30 seconds"));
    my_enum::print_traffic_light(red_light);
    let yellow_light = my_enum::trafficLight::Yellow(String::from("5 seconds"));
    my_enum::print_traffic_light(yellow_light);
    let green_light = my_enum::trafficLight::Green(String::from("20 seconds"));
    my_enum::print_traffic_light(green_light);
}
