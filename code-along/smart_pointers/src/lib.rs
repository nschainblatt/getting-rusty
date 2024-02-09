use std::ops::{Deref, DerefMut};

#[derive(Debug)]
pub enum List {
    Cons(i32, Box<List>),
    Nil,
}

pub struct MyBox<T>(T);

impl<T> MyBox<T> {
    pub fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
       &self.0
    }
}

impl<T> DerefMut for MyBox<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub fn dereference() {
    let x = 5;
    let y = &x;
    assert_eq!(x, *y);

    let z = Box::new(x);
    assert_eq!(x, *z);
}

pub fn dereference_my_box() {
    let x = 5;
    let y = MyBox::new(x);
    assert_eq!(x, *y);

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
