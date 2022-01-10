use std::fmt::Display;

fn main() {
    func1();
    func2();
    func3();
}

fn func1() {
    let string1 = String::from("abcd");

    {
        let string2 = "xyz";
        let result = longest(string1.as_str(), string2);
        println!("The longest string is {}", result);
    }
}

// lifetime annotation
//   - must start with an apostrophe
//   - is usually lowercase and very short name

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// NOTE: all string literals have static lifetime

fn func2() {
    struct ImportantExcerpt<'a> {
        // lifetime annotation is needed to hold reference in structs
        part: &'a str,
    }

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    println!("{}", i.part);
}

fn func3() {
    let string1 = String::from("abcd");
    let string2 = String::from("xyz");
    let announcement = 123;
    longest_with_an_announcement(&string1, &string2, announcement);
}

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
