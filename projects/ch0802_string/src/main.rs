fn main() {
    {
        let s1 = "initial contents".to_string();
        let s2 = String::from("initial contents");
        println!("{}", s1);
        println!("{}", s2);
    }
    {
        // UTF-8 encoded
        let s1 = String::from("1200");
        let s2 = String::from("一千两百");
        let s3 = String::from("🧑🎓🧑‍🎓");
        println!("{}", s1);
        println!("{}", s2);
        println!("{}", s3);
    }
    {
        let mut s = String::from("あか");
        s.push_str("むらさき");
        println!("{}", s);
    }
    {
        let mut s = String::from("あ");
        s.push('お');
        println!("{}", s);
    }
    {
        let s1 = String::from("Hello, ");
        let s2 = String::from("world!");

        // 1. `s1` is moved
        // 2. `s2` is not moved (borrowed)
        // 3. `s3` takes ownership of changed `s1` (added string of `s2`)
        let s3 = s1 + &s2;

        // println!("{}", s1); // this causes error
        println!("{}", s2);
        println!("{}", s3);
    }
    {
        let s1 = String::from("tic");
        let s2 = String::from("tac");
        let s3 = String::from("toe");

        // `format!` does not take ownership
        let s = format!("{}-{}-{}", s1, s2, s3);
        println!("{}", s);
        println!("{}", s1); // this does not cause error
        println!("{}", s2);
        println!("{}", s3);
    }
    {
        let s1 = String::from("hello");
        // Rust does not support index-access for string
        // let h = s1[0]; // this causes error
    }
    {
        let s1 = String::from("あいうえお");
        let len = s1.len();
        println!("len={}", len); // 15

        // let s2 = &s1[0..2];  // this causes panic
        let s3 = &s1[0..3];
        println!("{} (len={})", s3, s3.len()); // あ (len=3)
    }
    {
        let s = String::from("あいうえお");

        // バイト値
        for b in s.bytes() {
            println!("{}", b);
        }

        // 文字 (書記素クラスタとは異なる)
        for c in s.chars() {
            println!("{}", c);
        }

        // NOTE: 書記素クラスタを文字列から得る機能は標準ライブラリでは提供されない

        let s = String::from("🧑🎓🧑‍🎓");
        for b in s.bytes() {
            println!("{}", b);
        }
        for c in s.chars() {
            println!("{}", c);
        }
    }
}
