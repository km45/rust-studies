// MEMO: Is this requires?
// extern crate rand;

// MEMO: Why is this required?
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");

        // - variables are implicitly immutable in Rust,
        //   so make variables explicitly mutable with `mut` to be them mutable.
        let mut guess = String::new();

        std::io::stdin()
            // - `&` indicates "(immutable) reference" (implicitly immutable in Rust)
            // - `&mut` indicates "mutable reference"
            .read_line(&mut guess)
            // - `read_line` returns `io::Result`, which is enum (Ok, Err)
            // - `.expect` causes crash if `io::Result` equals `Err`.
            // - compiler warn if not call `expect` (different from C++)
            .expect("Failed to read line");

        // - convert String to u32
        // - shadow guess
        //   - use `guess` for u32 value though already exist `guess` for String
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // `println!` macro can treat placeholder `{}`
        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            std::cmp::Ordering::Less => println!("Too small"),
            std::cmp::Ordering::Equal => {
                println!("You Win!");
                break;
            }
            std::cmp::Ordering::Greater => println!("Too big!"),
        }
    }
}
