fn main() {
    {
        let mut s = String::from("hello");
        s.push_str(" world!");
        println!("{s}");
    }
    // println!("{s}"); // Memory of s is returned once it goes out of scope
    let s1 = String::from("hello");
    let length = calculate_length_1(s1);
    // println!("The length of '{}' is {}", s1, length); // s1 is moved to calculate_length_1 and is not available here

    let s2 = String::from("world");
    // & is a reference to the value of s2
    let length2 = calculate_length_2(&s2);
    println!("The length of '{}' is {}", s2, length2); // s2 is borrowed and is available here

    let mut s3 = String::from("hello");
    let length3 = calculate_length_3(&mut s3);
    println!("The length of '{}' is {}", s3, length3);
}

fn calculate_length_1(s: String) -> usize {
    s.len()
}

fn calculate_length_2(s: &String) -> usize {
    s.len()
    // s which is just the reference cannot be modified here
    // s.push_str(" world!");
}

fn calculate_length_3(s: &mut String) -> usize {
    s.push_str(" world!");
    s.len()
}
