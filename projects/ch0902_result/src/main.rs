use std::fs::File;
use std::io;
use std::io::ErrorKind;
use std::io::Read;

fn main() {
    {
        let f = File::open("hello.txt");
        let f = match f {
            Ok(file) => file,
            Err(ref error) if error.kind() == ErrorKind::NotFound => {
                match File::create("hello.txt") {
                    Ok(fc) => fc,
                    Err(e) => {
                        panic!("Tried to create file but there was a problem: {:?}", e)
                    }
                }
            }
            Err(error) => {
                panic!("There was a problem opening the file: {:?}", error)
            }
        };
    }
    {
        let f = File::open("hello2.txt").unwrap();
        let f = File::open("hello3.txt").expect("Failed to open hello3.txt");
    }
    {
        let s = read_hello4().unwrap();
        println!("{}", s);
    }
    {
        let s = read_hello5().unwrap();
        println!("{}", s);
    }
    {
        let s = read_hello6().unwrap();
        println!("{}", s);
    }
}

fn read_hello4() -> Result<String, io::Error> {
    let mut f = match File::open("hello4.txt") {
        Ok(file) => file,
        Err(e) => {
            return Err(e);
        }
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn read_hello5() -> Result<String, io::Error> {
    let mut f = File::open("hello5.txt")?; // use `?` operator
    let mut s = String::new();
    f.read_to_string(&mut s)?; // use `?` operator
    Ok(s)
}

fn read_hello6() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello6.txt")?.read_to_string(&mut s)?;
    Ok(s)
}
