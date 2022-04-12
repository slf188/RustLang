// This will allow dead code in the whole crate
#![allow(dead_code)]
use std::collections::HashMap;
// The parent modules distinguishes the two Result types
use std::fmt;
// Use of alias for Result
use std::io::Result as IoResult;
// Use lists
// use std::{cmp::Ordering, io};

// This can be shortened
// use std::io;
// use std::io::Write;
// We can shorten this to:
use std::io::{self, Write};
// Glob operator
use std::collections::*;

fn function1() -> fmt::Result {
    // --snip--
    Ok(())
}

fn function2() -> IoResult<()> {
    // --snip--
    Ok(())
}

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}
// Relative path with super
fn serve_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::serve_order();
    }

    fn cook_order() {}
    // Public struct in a module
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }
    // Public function in a module
    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
    // Public enum in a module
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

// Absolute path example
pub use crate::front_of_house::hosting; // Re-export with pub
// Relative path example
// use self::front_of_house::hosting;
// This code is unclear as to where add_to_waitlist is defined
// use crate::front_of_house::hosting::add_to_waitlist;

pub fn eat_at_restaurant() {
    crate::front_of_house::hosting::add_to_waitlist();
    front_of_house::hosting::add_to_waitlist();
    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
    let _order1 = back_of_house::Appetizer::Soup;
    let _order2 = back_of_house::Appetizer::Salad;
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}
