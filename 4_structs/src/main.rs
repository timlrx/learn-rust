#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

impl User {
    // &self is a reference to the instance of the struct
    fn username(&self) -> &String {
        &self.username
    }

    fn add_to_sign_in_count(&mut self) {
        self.sign_in_count += 1;
    }

    // Associated function - Self is an alias for the type that appears after the impl keyword
    fn new_user(username: String, email: String) -> Self {
        Self {
            active: true,
            username,
            email,
            sign_in_count: 0,
        }
    }
}

#[derive(Debug)]
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
    println!("user2: {user2:#?}");

    let col = Color(128, 128, 128);
    println!("{col:?}");

    user2.add_to_sign_in_count();
    println!("user2 username: {}", user2.username());
    println!("sign in counter: {}", user2.sign_in_count);

    let user3 = User::new_user(
        String::from("newuser789"),
        String::from("example@example.com"),
    );
    println!("user3: {user3:#?}");
}
