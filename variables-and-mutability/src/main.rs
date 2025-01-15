
const HELLO_WORLD: &str = "Hello World";

fn main() {
    // Immutable variables
    let apples = 50;
    let oranges = 14;

    // Declared as mutable variable
    let mut fruits = apples + oranges;
    
    println!("Total fruits {}", fruits);

    // Reassigning value to mutable fruits
    fruits = 18;

    println!("Total fruits {}", fruits);

    // Variable shadowing
    let grams_of_carbs = "120.45";
    let grams_of_carbs = 120.45;
    let mut grams_of_carbs = 120;
    grams_of_carbs = 121;

    // Showing usage of a constant
    println!("{HELLO_WORLD} there");
}
