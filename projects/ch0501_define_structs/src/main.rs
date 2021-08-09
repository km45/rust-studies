struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn print_user(user: &User) {
    println!("=== print_user ===");
    println!("username: {}", user.username);
    println!("email: {}", user.email);
    println!("sign_in_count: {}", user.sign_in_count);
    println!("active: {}", user.active);
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
fn print_color(color: &Color) {
    println!("Color = ({},{},{})", color.0, color.1, color.2);
}
fn print_point(point: &Point) {
    println!("Point = ({},{},{})", point.0, point.1, point.2);
}

fn main() {
    // struct
    {
        let user1 = build_user(
            String::from("someone@example.com"),
            String::from("someusername123"),
        );
        print_user(&user1);

        let user2 = User {
            email: String::from("another@example.com"),
            username: String::from("anotherusername567"),
            ..user1
        };
        print_user(&user2);
    }

    // tuple struct
    {
        let black = Color(0, 0, 0);
        let origin = Point(0, 0, 0);
        print_color(&black);
        print_point(&origin);
    }
}
