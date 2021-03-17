fn main() {
    println!("Hello, world!");
    println!();

    // call function defined not above but below
    another_function();
    println!();

    another_function2(5);
    println!();

    another_function3(5, 6);
    println!();

    statement_and_expression();
    println!();

    println!("{}", five());
    println!("{}", five2());
}

fn another_function() {
    println!("Another function.");
}

fn another_function2(x: i32) {
    println!("The value of x is: {}", x);
}

fn another_function3(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn statement_and_expression() {
    // statement does not return value
    let y = 6;
    println!("The value of y is: {}", y);

    // block is one of expressions
    let x = {
        let a = 1;
        a + 2 // no `;` for expression
    };
    println!("The value of x is: {}", x);
}

fn five() -> i32 {
    5
}

fn five2() -> i32 {
    // may use `return`
    return 5;
}
