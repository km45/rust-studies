use core::num;

fn main() {
    // match literals
    {
        let x = 1;

        match x {
            1 => println!("one"),
            2 => println!("two"),
            3 => println!("three"),
            _ => println!("anything"),
        }
    }

    // multiple patterns
    {
        let x = 1;

        match x {
            1 | 2 => println!("one or two"),
            3 => println!("three"),
            _ => println!("anything"),
        }
    }

    // ranges with `..=`
    {
        let x = 5;

        match x {
            1..=5 => println!("one through five"), // `1 | 2 | 3 | 4 | 5`
            _ => println!("something else"),
        }

        let x = 'c';

        match x {
            'a'..='j' => println!("early ASCII letter"),
            'k'..='z' => println!("late ASCII letter"),
            _ => println!("something else"),
        }
    }

    // destructure structs
    {
        struct Point {
            x: i32,
            y: i32,
        }

        let p = Point { x: 0, y: 7 };

        let Point { x, y } = p;
        assert_eq!(0, x);
        assert_eq!(7, y);

        match p {
            Point { x, y: 0 } => println!("On the x axis at {}", x),
            Point { x: 0, y } => println!("On the y axis at {}", y),
            Point { x, y } => println!("On neither axis: ({}, {})", x, y),
        }
    }

    // destructure enums
    {
        enum Message {
            Quit,
            Move { x: i32, y: i32 },
            Write(String),
            ChangeColor(i32, i32, i32),
        }

        let msg = Message::ChangeColor(0, 160, 255);

        match msg {
            Message::Quit => println!("The Quit variant has no data to destructure."),
            Message::Move { x, y } => {
                println!("Move in the x direction {} and in the y direction {}", x, y)
            }
            Message::Write(text) => println!("Text message: {}", text),
            Message::ChangeColor(r, g, b) => {
                println!("Change the color to red {}, green {} and blue {}", r, g, b)
            }
        }
    }

    // destructure references
    {
        struct Point {
            x: i32,
            y: i32,
        }

        let points = vec![
            Point { x: 0, y: 0 },
            Point { x: 1, y: 5 },
            Point { x: 10, y: -3 },
        ];

        let sum_of_squares: i32 = points.iter().map(|&Point { x, y }| x * x + y * y).sum();
        println!("sum = {}", sum_of_squares);
    }

    // ignore values in a pattern by `_` and `_x`
    {
        let s = Some(String::from("Hello!"));

        // - `_s` binds the value
        // - `_` does not bind the value
        if let Some(_) = s {
            println!("found a string");
        }

        println!("{:?}", s);
    }

    // ignore values in a pattern by `..`
    {
        struct Point {
            x: i32,
            y: i32,
            z: i32,
        }
        let origin = Point { x: 0, y: 0, z: 0 };
        match origin {
            Point { x, .. } => println!("x is {}", x),
        }

        let numbers = (2, 4, 8, 16, 32);
        match numbers {
            (first, .., last) => println!("Some numbers: {}, {}", first, last),
        }
    }

    // generate reference in a pattern using `ref` or `ref mut`
    {
        {
            let robot_name = Some(String::from("Bors"));

            match robot_name {
                Some(ref name) => println!("Found a name: {}", name),
                None => (),
            }
            println!("robot_name is: {:?}", robot_name);
        }

        {
            let mut robot_name = Some(String::from("Bors"));
            match robot_name {
                Some(ref mut name) => *name = String::from("Another name"),
                None => (),
            }
            println!("robot_name is: {:?}", robot_name);
        }
    }

    // match guards
    {
        {
            let num = Some(4);

            match num {
                Some(x) if x < 5 => println!("less than five: {}", x),
                Some(x) => println!("{}", x),
                None => (),
            }
        }
        {
            let x = Some(10);
            let y = 10;

            match x {
                Some(50) => println!("Got 50"),
                Some(n) if n == y => println!("Matched, n = {:?}", n),
                _ => println!("Default case, x = {:?}", x),
            }
        }
        {
            let x = 4;
            let y = false;

            match x {
                // `4 | 5 | 6 if y` equals `(4 | 5 | 6) if y`, not `4 | 5 | (6 if y)`
                4 | 5 | 6 if y => println!("yes"),
                _ => println!("no"),
            }
        }
    }

    // `@` bindings
    {
        enum Message {
            Hello { id: i32 },
        }

        let msg = Message::Hello { id: 5 };

        match msg {
            Message::Hello {
                id: id_variable @ 3..=7,
            } => {
                println!("Found an id in range: {}", id_variable)
            }
            Message::Hello { id: 10..=12 } => {
                println!("Found an id in another range")
            }
            Message::Hello { id } => {
                println!("Found some other id: {}", id)
            }
        }
    }
}
