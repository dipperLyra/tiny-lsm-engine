use std::{cell::RefCell, io, rc::Rc};

enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn main() {

    loop {
        accept_input();
    }
}

fn accept_input() {
    let mut key = String::new();
    let mut value = String::new();
    println!("Enter your key: ");
    io::stdin().read_line(&mut key).expect("Failed to read line");
    println!("Enter your value: ");
    io::stdin().read_line(&mut value).expect("Failed to read line");
    println!("You entered key: {} and value: {}", key.trim(), value.trim());
}

struct Node {
    value: i32,
    left_child: Rc<Node>,
    right_child: Rc<Node>,
}


