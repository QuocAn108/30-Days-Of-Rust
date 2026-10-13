use std::collections::HashMap;

pub fn example() {
    let mut inventory: Vec<String> = Vec::new();
    let mut quantities: HashMap<String, i32> = HashMap::new();
    // Add items
    inventory.push(String::from("Apples"));
    quantities.insert(String::from("Apples"), 10);
    quantities.insert(String::from("Bananas"), 10);
    // Update quantity
    *quantities.get_mut("Apples").unwrap() += 5;
    // Remove item
    inventory.retain(|item| item != "Bananas");
    // Print inventory
    for item in &inventory {
        let quantity = quantities.get(item).unwrap();
        println!("{}: {}", item, quantity);
    }
}
