// `!` represents `never type`.
// Functions that return never are called `diverging functions`.
fn bar() -> ! {
    panic!();
}

fn main() {
    // type aliases
    {
        type KiloMeters = i32;

        let x: i32 = 5;
        let y: KiloMeters = 4;

        println!("x + y = {}", x + y);
    }

    // dynamically sized types (DST)
    {
        // `str` is one of dynamically sized types (DST)
        "Hello there!"; // 12 bytes
        "How's it going?"; // 15 bytes

        let s1: &str = "Hello there!";
        let s2: &str = "How's it going?";

        // We cannot create variables of type `str`.
        // let s3: str = "Hello there!";
        // let s4: str = "How's it going?";
    }

    // Rust provides `Sized` trait to handle DST.
}
