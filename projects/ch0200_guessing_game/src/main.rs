fn main() {
    println!("Guess the number!");

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

    // `println!` macro can treat placeholder `{}`
    println!("You guessed: {}", guess);
}
