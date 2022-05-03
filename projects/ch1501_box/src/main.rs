fn main() {
    {
        let b = Box::new(5);
        println!("b = {}", b);
    }
    {
        use List::{Cons, Nil};
        // Lisp: (cons 1 (cons 2 (cons 3 nil)))
        let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
        println!("list = {:?}", list);
    }
}

#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}
