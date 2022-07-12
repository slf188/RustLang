// errors3.rs
// This is a program that is trying to use a completed version of the
// `total_cost` function from the previous exercise. It's not working though!
// Why not? What should we do to fix it?
// Execute `rustlings hint errors3` for hints!

// I AM NOT DONE

use std::num::ParseIntError;
// we need to modify the main function to return either Result or Option
// we can only return types that return termination, that is ()
// if we return string, i32, or any other type the program will persist
// we want main to be a function that concludes the program
fn main() -> Result<(), ParseIntError> {
    let mut tokens = 100;
    let pretend_user_input = "8";

    let cost = total_cost(pretend_user_input)?;

    if cost > tokens {
        println!("You can't afford that many!");
        // this case is ok so return Ok()
        Ok(())
    } else {
        tokens -= cost;
        println!("You now have {} tokens.", tokens);
        // this case is also ok so return the same Ok()
        Ok(())
    }
}

pub fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
    let processing_fee = 1;
    let cost_per_item = 5;
    let qty = item_quantity.parse::<i32>()?;

    Ok(qty * cost_per_item + processing_fee)
}

