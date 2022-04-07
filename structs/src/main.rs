#[allow(dead_code)] // This will allow unused variables to exist in our Rust programs.
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
// Tuple structs
struct Color(i32, i32, i32);
// Unit-like struct wihout any fields.
struct AlwaysEqual;
#[derive(Debug)] // This will allow us to print out the structs in a more readable format.
struct Rectangle {
    width: u32,
    height: u32,
}
// Implementing multiple methods for the Rectangle struct.
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }
}
// Implementing a valid method for the Rectangle struct.
impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    user1.email = String::from("anotheremail@example.com");
    // Struct update syntax
    let _user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };
    // Tuple structs
    let _black = Color(0, 0, 0);
    // Unit-like struct instance
    let _subject = AlwaysEqual;
    // Structs with methods
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("rect1 is {:?}", rect1);
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}