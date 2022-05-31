use core::fmt;
use std::ops::Add;

use ch1902_advanced_traits::Counter;
use ch1902_advanced_traits::{Meters, Millimeters};

fn main() {
    {
        let mut counter = Counter::new();
        assert_eq!(counter.next(), Some(1));
        assert_eq!(counter.next(), Some(2));
        assert_eq!(counter.next(), Some(3));
        assert_eq!(counter.next(), Some(4));
        assert_eq!(counter.next(), Some(5));
        assert_eq!(counter.next(), None);
    }
    {
        let p1 = Point { x: 1, y: 0 };
        let p2 = Point { x: 2, y: 3 };

        let p3 = p1 + p2;
        assert_eq!(p3.x, 3);
        assert_eq!(p3.y, 3);
    }
    {
        let sum = Millimeters::new(123) + Meters::new(456);
        println!("sum = {:?}", sum);
    }
    {
        let person = Human;
        Pilot::fly(&person);
        Wizard::fly(&person);
        person.fly();
    }
    {
        println!("A baby dog is called a {}", Dog::baby_name());

        // Rust compiler treats `Animal::baby_name()` as error.
        // println!("Animal::baby_name = {}", Animal::baby_name());

        // use `fully qualified syntax`
        println!("A baby dog is called a {}", <Dog as Animal>::baby_name());
    }
    {
        let p = Point { x: 1, y: 3 };
        p.outline_print();
    }
    {
        // We can implement a trait on a type if the trait or the type are "local".
        // We can avoid this restriction using `newtype pattern`.

        struct Wrapper(Vec<String>); // local

        impl fmt::Display for Wrapper {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "[{}]", self.0.join(", "))
            }
        }

        let w = Wrapper(vec![String::from("hello"), String::from("world")]);
        println!("w = {}", w);
    }
}

struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up.");
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

// use `supertrait`
trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();

        println!("**{}**", "*".repeat(len));
        println!("* {} *", " ".repeat(len));
        println!("* {} *", output);
        println!("* {} *", " ".repeat(len));
        println!("**{}**", "*".repeat(len));
    }
}

impl OutlinePrint for Point {}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
