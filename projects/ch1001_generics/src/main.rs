struct Point<T, U> {
    x: T,
    y: U,
}

// NOTE: We can use generics for enum like below.
//
// enum Option<T> {
//     Some(T),
//     None,
// }
// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }

impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }
}

// Implement method only for T=i32 and U=32
impl Point<f32, f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    {
        let both_integer = Point { x: 1, y: 2 };
        println!("x: {}, y: {}", both_integer.x, both_integer.y);

        let both_float = Point { x: 3.4, y: 5.6 };
        println!("x: {}, y: {}", both_float.x, both_float.y);

        let integer_and_float = Point { x: 7, y: 8.9 };
        println!("x: {}, y: {}", integer_and_float.x, integer_and_float.y);
    }
    {
        let both_integer = Point { x: 1, y: 2 };
        let both_float = Point { x: 3.4, y: 5.6 };

        both_integer.x();
        both_float.x();

        // both_integer.distance_from_origin();  // this causes error
        both_float.distance_from_origin();
    }
    {
        let p1 = Point { x: 5, y: 10.4 };
        let p2 = Point { x: "Hello", y: 'c' };
        let p3 = p1.mixup(p2);
        println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
    }
}
