#![allow(dead_code)]

fn main() {
    {                      // s is not valid here, it’s not yet declared
        let _s = "hello";   // s is valid from this point forward
        // do stuff with s
    }                      // this scope is now over, and s is no longer valid
    let mut s = String::from("hello");
    s.push_str(", world!"); // push_str() appends a literal to a String
    println!("{}", s); // This will print `hello, world!`
    // This code works because by default integers have a known size at compile time.
    let x = 5;
    let _y = x;
    // We now have two variables, x and y, and both equal 5.
    // These two values are pushed onto the stack.
    // The string version:
    // s1 is no longer valid because it was borrowed
    let s1 = String::from("hello");
    let _s2 = s1;
    // The only way to make s1 available after borrowing is by doing:
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);
    let s = String::from("hello");
    takes_ownership(s);
    makes_copy(x);
    let _s1 = gives_ownership();
    let _s2 = String::from("hello");
    let _s3 = takes_and_gives_back(s2);   
    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);
    println!("The length of '{}' is {}.", s2, len);
    let s1 = String::from("hello");
    let len = calculating_length(&s1);
    println!("The length of '{}' is {}.", s1, len);
    // Mutable references
    let mut s = String::from("hello");
    change(&mut s);
    // String slices
    let s = String::from("hello world");
    let _hello = &s[0..5];
    let _world = &s[6..11];
}

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}
// References and borrowing:
fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}

fn calculating_length(s: &String) -> usize {
    s.len()
}
// Mutable references
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}