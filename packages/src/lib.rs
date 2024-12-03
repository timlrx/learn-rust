mod garden;

use garden::{fruits, vegetables};

// Re-exporting Names with pub use
pub use garden::vegetables::print_vegetables;
pub use garden::vegetables::Vegetable;

pub fn print_all() {
    println!("Hello, world!");
    garden::print_garden();
    vegetables::print_vegetables(Vegetable::Tomato);
    fruits::print_fruits();
}

fn print_animal() {
    println!("This is an animal");
}
