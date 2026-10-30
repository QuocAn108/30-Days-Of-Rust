pub mod shapes;

pub fn ex1() {
    println!("=== Exercise: Level 1 ===");

    // 1 & 2. Call circle_area from module shapes and print the result
    let radius = 5.0;
    let area = shapes::circle_area(radius);
    println!("Shapes module: Circle area with radius {} is {:.2}", radius, area);

    let radius_small = 2.5;
    println!(
        "Shapes module: Circle area with radius {} is {:.2}",
        radius_small,
        shapes::circle_area(radius_small)
    );

    // 3. Use unit_converter library crate
    let meters = 2500.0;
    let km = unit_converter::meters_to_kilometers(meters);
    println!("Unit Converter: {} meters = {} kilometers", meters, km);

    let km_val = 4.2;
    let m = unit_converter::kilometers_to_meters(km_val);
    println!("Unit Converter: {} kilometers = {} meters", km_val, m);
    println!();
}
