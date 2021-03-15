fn main() {
    {
        // - immutable variables and constants are different
        // - type annotations (ref: ch0302) are required for constants
        const INITIAL_VALUE: u32 = 5;

        // variables are immutable by default
        let mut x = INITIAL_VALUE;

        println!("The value of x is: {}", x);
        x = 6;
        println!("The value of x is: {}", x);
    }

    {
        let x = 1;
        println!("The value of x is: {}", x);
        let x = 2; // not mutable variable but shadowing
        println!("The value of x is: {}", x);
        let x = x + 1;
        println!("The value of x is: {}", x);
    }

    {
        let spaces = "    ";
        println!("The value of spaces is: [{}]", spaces);
        let spaces = spaces.len(); // shadowing can use different type
        println!("The value of spaces is: [{}]", spaces);
    }
}
