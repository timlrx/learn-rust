pub enum Vegetable {
    Tomato,
    Potato,
    Carrot,
    Other(String),
}

pub fn print_vegetables(veg: Vegetable) {
    match veg {
        Vegetable::Tomato => println!("This is a tomato"),
        Vegetable::Potato => println!("This is a potato"),
        Vegetable::Carrot => println!("This is a carrot"),
        Vegetable::Other(name) => println!("This is a {}", name),
    }
}
