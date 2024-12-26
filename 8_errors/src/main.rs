use std::fs::File;
use std::io::{self, Read};

fn main() {
    let username = read_username_from_file();
    match username {
        Ok(username) => println!("Username: {}", username),
        Err(e) => println!("Error: {}", e),
    }
}

fn read_username_from_file() -> Result<String, io::Error> {
    // Shortcut of propagating errors - ? operator
    //
    // let username_file_result = File::open("hello.txt");
    // let mut username_file = match username_file_result {
    //     Ok(file) => file,
    //     Err(e) => return Err(e),
    // };

    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;

    // Or a one-liner
    // File::open("hello.txt")?.read_to_string(&mut username)?;

    Ok(username)
}
