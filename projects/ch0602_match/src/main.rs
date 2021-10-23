#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // and so on
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    println!("Penny is {} cent(s)", value_in_cents(Coin::Penny));
    println!(
        "Quarter is {} cent(s)",
        value_in_cents(Coin::Quarter(UsState::Alabama))
    );

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    let u8_value = 0u8;
    match u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (), // nothing to do
    }
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
