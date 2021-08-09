fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn main() {
    let s = first_word("Hello, world!");
    println!("{}", s);

    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..(a.len() - 1)];
    for i in slice.iter() {
        println!("{}", i);
    }
}
