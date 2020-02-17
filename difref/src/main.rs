struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}
#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

use std::ops::Deref;
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}
use std::cell::RefCell;
use std::rc::Rc;

enum ReferenceList {
    Cons(i32, Rc<ReferenceList>),
    Nil,
}
impl ReferenceList {
    fn tail(&self) -> Option<&Rc<ReferenceList>> {
        match *self {
            ReferenceList::Cons(_, ref item) => Some(item),
            ReferenceList::Nil => None,
        }
    }
}
fn main() {
    let c = CustomSmartPointer {
        data: String::from("hello, world"),
    };
    let b = Box::new(4);
    println!("{}", b);
    let x = 5;
    let y = MyBox::new(x);
    assert_eq!(5, *y);
    let m = MyBox::new(String::from("yochidros"));
    hello(&m);

    let c = List::Cons(
        1,
        Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))),
    );
    println!("{:?}", c);
    println!("created smart pointer");

    let a = Rc::new(ReferenceList::Cons(
        5,
        Rc::new(ReferenceList::Cons(10, Rc::new(ReferenceList::Nil))),
    ));
    println!("{}", Rc::strong_count(&a));
    {
        let c = ReferenceList::Cons(4, Rc::clone(&a));
        println!("{}", Rc::strong_count(&a));
    }
}
fn hello(name: &str) {
    println!("Hello, {}", name);
}
