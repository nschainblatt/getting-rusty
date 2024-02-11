use smart_pointers::{
    self,
    List::{Cons, Nil},
    List2::{Cons2, Nil2},

use std::rc::Rc;

fn main() {
    let b = Box::new(5);
    println!("{b}");

    let list = Cons(1, Box::new(Cons(2, Box::new(Nil))));
    println!("{list:#?}");

    // Using List2 which uses the reference count smart pointer to allow a list to have multiple owners
    // We keep track of the strong count of how many references 'a' has, this is a way of allowing multiple owners
    // and ensuring that the referenced value does not get cleaned up until the reference count is 0
    let a = Rc::new(Cons2(1, Rc::new(Cons2(2, Rc::new(Nil2)))));
    println!("Count after creating list 'a': {}", Rc::strong_count(&a));
    let b = Cons2(3, Rc::clone(&a));
    println!("Count after creating list 'b': {}", Rc::strong_count(&a));
    {
        let c = Cons2(4, Rc::clone(&a));
        println!("Count after creating list 'c': {}", Rc::strong_count(&a));
    }
    println!("Count after list 'c' goes out of scope: {}", Rc::strong_count(&a));
    println!("{:?}\n{:?}\n", a, b);

    smart_pointers::dereference();
    smart_pointers::dereference_my_box();
    smart_pointers::deref_coercion();
    let smart_pointer = smart_pointers::CustomSmartPointer::build("Store this".to_string());
    println!("\nCustomSmartPointer created.");
    println!("CustomSmartPointer instance will be dropped once it goes out of scope! Data: {}", smart_pointer.data);

    // Using std::mem::drop to drop a value before it goes out of scope
    let smart_pointer_2 = smart_pointers::CustomSmartPointer::build("And this".to_string());
    println!("\nCustomSmartPointer created.");
    drop(smart_pointer_2);
    println!("CustomSmartPointer has been dropped before going out of scope.");


}
