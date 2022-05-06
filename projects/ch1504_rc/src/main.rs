// `Rc<T>` is reference counting smart pointer used for "single" thread.

use std::rc::Rc;

fn main() {
    use List::{Cons, Nil};

    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("a = {:?}", a);
    println!("count after creating a = {}", Rc::strong_count(&a));

    // `Rc::clone` is not deep-copy but increment reference counting
    let b = Cons(3, Rc::clone(&a));
    println!("b = {:?}", b);
    println!("count after creating b = {}", Rc::strong_count(&a));

    {
        let c = Cons(4, Rc::clone(&a));
        println!("c = {:?}", c);
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}

#[derive(Debug)]
enum List {
    Cons(i32, Rc<List>),
    Nil,
}
