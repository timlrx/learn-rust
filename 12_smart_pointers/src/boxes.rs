#[derive(Debug)]
pub enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::boxes::List::{Cons, Nil};

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

use std::ops::Deref;

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn hello(name: &str) {
    println!("Hello, {name}!");
}

pub fn box_example() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("{:?}", list);
    let m = MyBox::new(String::from("Rust"));
    hello(&m);
    // &MyBox<String> -> &String -> &str (deref coercion)
    /*
    Equivalent to  hello(&(*m)[..]);
       The (*m) dereferences the MyBox<String> into a String.
       Then the & and [..] take a string slice of the String that is equal to the whole string to match the signature of hello.
    */
}
