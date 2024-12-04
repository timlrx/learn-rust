#![allow(unused_variables)]
#![allow(dead_code)]
fn main() {
    println!("\n=== Strings ===");
    example_string();
    println!("\n=== Hashmaps ===");
    example_hashmap();
    println!("\n=== Vectors ===");
    example_vector();
}

fn example_string() {
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
    println!("{}", s3);

    // Instead of indexing characters, use chars() method
    for c in "Зд".chars() {
        println!("{c}");
    }
}

fn example_hashmap() {
    use std::collections::HashMap;

    #[derive(Hash, Eq, PartialEq, Debug)]
    enum Fruits {
        Apple,
        Banana,
        Others(String),
    }

    let mut fruit_scores: HashMap<Fruits, i32> = HashMap::new();
    // Hashmap will take ownership of the key and value
    fruit_scores.insert(Fruits::Apple, 10);
    fruit_scores.insert(Fruits::Banana, 20);
    fruit_scores.insert(Fruits::Others(String::from("Pineapple")), 30);

    let banana_score = fruit_scores.get(&Fruits::Banana).copied().unwrap_or(0);

    // Update the value of an existing key
    fruit_scores.insert(Fruits::Banana, banana_score + 10);

    // Insert if the key does not exist
    fruit_scores
        .entry(Fruits::Others(String::from("Pineapple")))
        .or_insert(40);

    for (fruit, score) in &fruit_scores {
        println!("{:?}: {}", fruit, score);
    }
}

fn example_vector() {
    let mut v = Vec::new();
    let v2 = vec![1, 2, 3, 4, 5];
    v.push(1);
    v.push(2);
    v.push(3);
    v.push(4);
    v.push(5);
    println!("{:?}", v);

    let fourth = &v[3];
    println!("The fourth element is {}", fourth);
    let value = v.get(6);
    match value {
        Some(v) => println!("The value is {}", v),
        None => println!("There is no specified element"),
    }

    v.push(6);
    let mut sum = 0;
    for i in &v {
        sum += i;
        println!("{sum}");
    }

    let mut v3 = vec![String::from("Hello "), String::from("!")];
    // Non-copyable types cannot be moved out of a vector by indexing
    // let s = v3[0];
    let mut s = Vec::remove(&mut v3, 0);
    s.push_str("World!");
    println!("{}", s);
    println!("{:?}", v3);

    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    // Use enum to store different types in a vector
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    println!("{:?}", row);
}
