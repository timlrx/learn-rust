pub enum List {
    Cons(i32, Rc<List>),
    Nil,
}

#[derive(Debug)]
pub enum MutableList {
    MCons(Rc<RefCell<i32>>, Rc<MutableList>), // RefCell allows for interior mutability
    MNil,
}

use crate::rcs::List::{Cons, Nil};
use crate::rcs::MutableList::{MCons, MNil};
use std::{cell::RefCell, rc::Rc};

pub fn rcs_example() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a)); // Rc::clone does not make a deep copy, only increments the reference count
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));

    println!("Using Rc with RefCell for interior mutability");
    let value = Rc::new(RefCell::new(5));
    let a = Rc::new(MCons(Rc::clone(&value), Rc::new(MNil)));
    let b = MCons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = MCons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    *value.borrow_mut() += 10;
    // Will panic
    // let x = value.borrow_mut();
    // let y = value.borrow_mut();

    println!("a after = {a:?}");
    println!("b after = {b:?}");
    println!("c after = {c:?}");
}
