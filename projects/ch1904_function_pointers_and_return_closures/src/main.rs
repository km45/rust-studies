fn add_one(x: i32) -> i32 {
    x + 1
}

// function pointer `fn` implements closure traits `Fn`, `FnMut` and `FnOnce`.
fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn main() {
    {
        let answer = do_twice(add_one, 5);
        println!("The answer is: {}", answer);
    }
    {
        let list_of_numbers = vec![1, 2, 3];
        let list_of_strings: Vec<String> = list_of_numbers.iter().map(|i| i.to_string()).collect();

        for s in list_of_strings {
            println!("{}", s);
        }

        let list_of_strings: Vec<String> =
            list_of_numbers.iter().map(ToString::to_string).collect();
        for s in list_of_strings {
            println!("{}", s);
        }
    }
}

// We cannot return closures because cannot determine the size of closures. (DST)
// fn returns_closure() -> Fn(i32) -> i32 {
//     |x| x + 1
// }
fn returns_closure() -> Box<Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}
