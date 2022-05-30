use core::slice;

fn main() {
    {
        let mut num = 5;

        // create row pointers in safe code
        let r1 = &num as *const i32;
        let r2 = &mut num as *mut i32;

        unsafe {
            // dereference row pointers and access the data being pointed to in unsafe code
            println!("r1 is: {}", *r1);
            println!("r2 is: {}", *r2);
        }
    }
    {
        unsafe {
            // call `unsafe` function in unsafe code
            dangerous();
        }
    }
    {
        let mut v = vec![1, 2, 3, 4, 5, 6];

        let r = &mut v[..];

        // let (a, b) = r.split_at_mut(3);
        let (a, b) = split_at_mut(r, 3);

        assert_eq!(a, &mut [1, 2, 3]);
        assert_eq!(b, &mut [4, 5, 6]);
    }
    {
        unsafe {
            // call C function using FFI (Foreign Function Interface)
            println!("Absolute value of -3 according to C: {}", abs(-3));
        }
    }
    {
        // Rust treats global static mutable variable as unsafe
        unsafe {
            COUNTER += 1;
        }
        unsafe {
            println!("COUNTER: {}", COUNTER);
        }
    }
}

unsafe fn dangerous() {}

fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();

    assert!(mid <= len);

    // (&mut slice[..mid], &mut [mid..])

    let ptr = slice.as_mut_ptr();
    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.offset(mid as isize), len - mid),
        )
    }
}

// use "C" ABI (Application Binary Interface)
extern "C" {
    fn abs(input: i32) -> i32;
}

// create an interface to call Rust functions from other languages
#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}

// global variable
static mut COUNTER: u32 = 0;

// unsafe traits
unsafe trait Foo {}
unsafe impl Foo for i32 {}
