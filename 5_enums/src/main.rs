#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}
#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> Option<i32> {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            Some(1)
        }
        Coin::Nickel => Some(5),
        // Catch all other coins
        other => {
            println!("Not a penny or nickel: {:?}", other);
            None
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("Home: {:?}", home);
    println!("Loopback: {:?}", loopback);
    println!("Value in cents: {:?}", value_in_cents(Coin::Dime));
}
