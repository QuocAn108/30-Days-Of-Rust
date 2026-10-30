use rand::seq::SliceRandom;
use rand::Rng;

fn main() {
    println!("=== Random Generator Crate (using `rand`) ===");

    let mut rng = rand::thread_rng();

    // 1. Random integer in range
    let secret_number: i32 = rng.gen_range(1..=100);
    println!("Random integer (1..=100): {}", secret_number);

    // 2. Rolling a dice (1 to 6)
    let dice_roll: u32 = rng.gen_range(1..=6);
    println!("Dice roll: 🎲 {}", dice_roll);

    // 3. Random float
    let random_float: f64 = rng.gen_range(0.0..1.0);
    println!("Random float (0.0..1.0): {:.4}", random_float);

    // 4. Random boolean (coin flip)
    let coin_flip: bool = rng.gen_bool(0.5);
    println!("Coin flip (heads = true): {}", coin_flip);

    // 5. Random selection from an array / slice
    let greetings = ["Hello", "Xin chào", "Bonjour", "Hola", "Ciao", "Konnichiwa"];
    if let Some(greeting) = greetings.choose(&mut rng) {
        println!("Random greeting: {}", greeting);
    }

    // 6. Shuffling a vector
    let mut cards = vec!["Ace", "King", "Queen", "Jack", "10"];
    cards.shuffle(&mut rng);
    println!("Shuffled cards: {:?}", cards);
}
