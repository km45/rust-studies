fn main() {
    // `match` arm
    {
        let favorite_color: Option<&str> = None;
        match favorite_color {
            Some(s) => println!("favorite_color = {}", s),
            _ => println!("favorite_color is None"),
        }
    }

    // conditional `if let` expression
    {
        let favorite_color: Option<&str> = None;
        let is_tuesday = false;
        let age: Result<u8, _> = "34".parse();

        if let Some(color) = favorite_color {
            println!("Using your favorite color, {}, as the background", color);
        } else if is_tuesday {
            println!("Tuesday is green day!");
        } else if let Ok(age) = age {
            if age > 30 {
                println!("Using purple as the background color");
            } else {
                println!("Using orange as the background color");
            }
        } else {
            println!("Using blue as the background color");
        }
    }

    // `while let` conditional loop
    {
        let mut stack = vec![1, 2, 3];

        while let Some(top) = stack.pop() {
            println!("{}", top);
        }
    }

    // `for` loop
    {
        let v = vec!['a', 'b', 'c'];

        // in `for x in y`, `x` is the pattern
        for (index, value) in v.iter().enumerate() {
            println!("{} is at index {}", value, index);
        }
    }

    // let statement
    {
        // in `let x = y`, `x` is the pattern
        let (a, _, c) = (1, 2, 3);

        println!("a = {}, c = {}", a, c);
    }
}
