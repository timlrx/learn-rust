struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color(i32, i32, i32);

fn main() {
    // Entire instance must be mutatable
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
    println!("{}", user1.username);
    user1.username = String::from("newusername123");
    println!("{}", user1.username);

    let mut user2 = User {
        username: String::from("anotherusername456"),
        ..user1 // Copy the rest of the fields from user1
    };
    // println!("{}", user1.email); // Email is not copied, but moved from user1 to user2
    println!("{}", user1.sign_in_count);
    println!("{}", user2.email);

    let col = Color(128, 128, 128);
    println!("{}", col.0);
}
