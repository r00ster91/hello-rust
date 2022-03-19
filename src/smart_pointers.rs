enum List {
    Cons(i32, Box<List>),
    Nil,
}

use List::{Cons, Nil};

pub fn main() {
    // use Box to store i32 on the heap
    let b = Box::new(5);
    println!("b = {}", b);

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}

// I didn't interact much with the rest of the Smart Pointers chapter
