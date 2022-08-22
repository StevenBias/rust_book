use std::ops::Deref;

use crate::List::{Cons, Nil};

enum List {
    Cons(i32, Box<List>),
    Nil,
}

struct MyBox<T>(T);

struct CustomSmartPointer {
    data: String,
}

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data '{}'", self.data);
    }
}

fn test_box() {
    let list = Cons(1,
                    Box::new(Cons(2,
                                  Box::new(Cons(3,
                                                Box::new(Nil))))));
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}

fn test_deref() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    let m = MyBox::new(String::from("Rust"));
    hello(&m);
}

fn main() {
    test_box();
    test_deref();

    let c = CustomSmartPointer{ data: String::from("My stuff")};
    let d = CustomSmartPointer{ data: String::from("other stuff")};
    println!("CustomSmartPointer Created");
}
