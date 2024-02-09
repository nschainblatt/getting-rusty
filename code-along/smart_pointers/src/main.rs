use smart_pointers::{
    self,
    List::{Cons, Nil},
};

fn main() {
    let b = Box::new(5);
    println!("{b}");

    let list = Cons(1, Box::new(Cons(2, Box::new(Nil))));
    println!("{list:#?}");

    smart_pointers::dereference();
    smart_pointers::dereference_my_box();
    smart_pointers::deref_coercion();
}
