#[derive(Debug)]
enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn main() {
    let home = IpAddrKind::V4(127, 0, 0, 1);
    let loopback = IpAddrKind::V6(String::from("::1"));

    println!("home is {:?}", home);
    println!("loopback is {:?}", loopback);

    let some_number = Some(5);
    let absent_number: Option<i32> = None;
}
