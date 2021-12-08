fn main() {
    {
        let v1: Vec<i32> = Vec::new();
        let v2 = vec![1, 2, 3];
    }
    {
        // compiler can infer the type
        let mut v = Vec::new(); // Vec<i32>
        v.push(5);
        v.push(6);
        v.push(7);
        v.push(8);
    }
    {
        let v = vec![1, 2, 3, 4, 5];

        let third = &v[2]; // &i32
        println!("{}", third);
        let third = v.get(2); // Option<&i32>
        match third {
            None => println!("third is None"),
            Some(i) => println!("third is {}", i),
        }

        // let dose_not_exist = &v[100]; // this causes panic
        let dose_not_exist = v.get(100);
        match dose_not_exist {
            None => println!("dose_not_exist is None"),
            Some(i) => println!("dose_not_exist is {}", i),
        }
    }
    {
        // treat as error by borrow checker if mutable and immutable borrows exist
        let mut v = vec![1, 2, 3, 4, 5];
        let first = &v[0];

        // this requires mutable reference
        v.push(6);

        //this requires immutable reference
        // println!("The first element is {}", first);
    }
    {
        let v = vec![100, 32, 57];
        for i in &v {
            println!("{}", i);
        }
    }
    {
        let mut v = vec![100, 32, 57];
        for i in &mut v {
            *i += 50;
        }
        for i in &v {
            println!("{}", i);
        }
    }
    {
        enum SpreadsheetCell {
            Int(i32),
            Float(f64),
            Text(String),
        }
        let row = vec![
            SpreadsheetCell::Int(3),
            SpreadsheetCell::Text(String::from("blue")),
            SpreadsheetCell::Float(10.12),
        ];
        for i in &row {
            match i {
                SpreadsheetCell::Int(x) => println!("Int: {}", x),
                SpreadsheetCell::Float(x) => println!("Float: {}", x),
                SpreadsheetCell::Text(x) => println!("Text: {}", x),
            }
        }
    }
}
