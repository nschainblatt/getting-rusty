use std::ops::{Deref, DerefMut};
use std::rc::Rc;

#[derive(Debug)]
pub enum List {
    Cons(i32, Box<List>),
    Nil,
}

#[derive(Debug)]
pub enum List2 {
   Cons2(i32, Rc<List2>),
   Nil2,
}

#[derive(Debug)]
pub struct MyBox<T>(T);

impl<T> MyBox<T> {
    pub fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

// Used to dereferencing immutable versions of the MyBox smart pointer
impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// Used to dereferencing mutable versions of the MyBox smart pointer
impl<T> DerefMut for MyBox<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub fn dereference() {
    let x = 5;
    let y = &x;
    println!("{}", *y.deref());
    assert_eq!(x, *y.deref());

    // Rust runs the above line behind the scenes
    assert_eq!(x, *y);

    let z = Box::new(x);
    assert_eq!(x, *z);
}

pub fn dereference_my_box() {
    let x = 5;
    let y = MyBox::new(x);
    println!("{}", *y.deref());
    assert_eq!(x, *y.deref());

    // Rust runs the above line behind the scenes
    assert_eq!(x, *y);

    // Testing with an owned type
    let a = "A String type".to_string();
    let b = MyBox(a.clone());

    println!("{:?}", b);
    assert_eq!(a, *b);
    println!("{}", *b);

    let mut z = 5;
    let mut w = MyBox::new(z);
    z += 1;
    *w += 1;
    assert_eq!(z, *w);
}

pub fn deref_coercion() {
    let b = MyBox("Rust".to_string());
    // &String to &str
    hello(&b);
}

pub fn hello(word: &str) {
    println!("{word}");
}

// Implementing the Drop trait on a new custom smart pointer we create

pub struct CustomSmartPointer {
    pub data: String
}

impl CustomSmartPointer {
    pub fn build(data: String) -> CustomSmartPointer {
        CustomSmartPointer { data }
    }
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping a CustomSmartPointer instance. Data: {}", self.data);
    }
}