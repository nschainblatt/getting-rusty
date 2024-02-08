use smart_pointers::List::{Cons, Nil};

fn main() {
    let b = Box::new(5);
    println!("{b}");

    let list = Cons(1, Box::new(Cons(2, Box::new(Nil))));
}

