pub mod vegetables;

use vegetables::Vegetable;

pub fn print_garden() {
    println!("This is a garden");
    vegetables::print_vegetables(Vegetable::Other("cabbage".to_string()));
}

pub mod fruits {
    pub fn print_fruits() {
        println!("This is a fruit");
    }
}
