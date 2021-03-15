fn main() {
    {
        let min: u8 = 0;
        let max: u8 = 255;
        println!("{}, {}", min, max);
        let min: u16 = 0;
        let max: u16 = 65535;
        println!("{}, {}", min, max);
        let min: u32 = 0;
        let max: u32 = 4294967295;
        println!("{}, {}", min, max);
        let min: u64 = 0;
        let max: u64 = 18446744073709551615;
        println!("{}, {}", min, max);

        // signed integers are represented by two's complement
        let min: i8 = -128;
        let max: i8 = 127;
        println!("{}, {}", min, max);
        let min: i16 = -32768;
        let max: i16 = 32767;
        println!("{}, {}", min, max);
        let min: i32 = -2147483648;
        let max: i32 = 2147483647;
        println!("{}, {}", min, max);
        let min: i64 = -9223372036854775808;
        let max: i64 = 9223372036854775807;
        println!("{}, {}", min, max);

        let x: usize = 1;
        println!("{}", x);
        let x: isize = 1;
        println!("{}", x);
    }
    {
        let x = 0x10;
        println!("{}", x);
        let x = 0o10;
        println!("{}", x);
        let x = 0b10;
        println!("{}", x);

        // may use `_` to improve readability
        let x = 1_000_000;
        println!("{}", x);
        let x = 100_0000;
        println!("{}", x);
    }
    {
        // floating point numbers are represented by IEEE-754
        let x: f32 = 1.23;
        println!("{}", x);
        let x: f64 = 1.23;
        println!("{}", x);
    }
    {
        let sum = 5 + 10;
        println!("{}", sum);

        let difference = 5 - 10;
        println!("{}", difference);

        let product = 4 * 30;
        println!("{}", product);

        let quotient = 56.7 / 32.2;
        println!("{}", quotient);
        let quotient = 43 / 5;
        println!("{}", quotient);

        let remainder = 43 % 5;
        println!("{}", remainder);
    }
    {
        let t: bool = true;
        println!("{}", t);
        let f: bool = false;
        println!("{}", f);
    }
    {
        let c: char = 'A';
        println!("{}", c);
        let c: char = 'あ';
        println!("{}", c);
        let c: char = '亜';
        println!("{}", c);
    }
    {
        // tuple
        let tup: (i32, f64, u8) = (-1, 1.5, 2);
        let (x, y, z) = tup;
        println!("{}, {}, {}", x, y, z);
        println!("{}, {}, {}", tup.0, tup.1, tup.2);
    }
    {
        // fixed length array
        let a = [1, 2, 3, 4, 5];
        println!("{}, {}, {}, {}, {}", a[0], a[1], a[2], a[3], a[4]);
    }
}
