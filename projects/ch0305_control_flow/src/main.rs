fn main() {
    if_expression(3);
    if_expression(7);
    println!();

    if_expression2(6);
    if_expression2(12);
    if_expression2(5);
    println!();

    use_if_as_rhs_for_let_expression();
    println!();

    loop_with_while();
    println!();

    loop_with_for();

    // infinite_loop();
}

fn if_expression(number: i32) {
    // condition must be `bool`
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
}

fn if_expression2(number: i32) {
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else {
        println!("number is not divisible by 4 or 3");
    }
}

fn use_if_as_rhs_for_let_expression() {
    let condition = true;
    let number = if condition { 5 } else { 6 }; // types must be same for all arms

    println!("The value of number is: {}", number);
}

fn infinite_loop() {
    loop {
        println!("again!");
    }
}

fn loop_with_while() {
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number = number - 1;
    }

    println!("LIFTOFF!!!");
}

fn loop_with_for() {
    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("The value is: {}", element);
    }

    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}
