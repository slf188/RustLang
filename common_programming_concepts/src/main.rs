fn main() {
    // Variables
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
    // Constants
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("{}", THREE_HOURS_IN_SECONDS);
    // Shadowing
    let y = 7;
    let y = y + 1;
    println!("The value of x with shadowing is: {}", y);
    // Data types
    let _z = 2.0; // f64
    let _k: f32 = 3.0; // f32
                      // addition
    let _sum = 5 + 10;
    // subtraction
    let _difference = 95.5 - 4.3;
    // multiplication
    let _product = 4 * 30;
    // division
    let _quotient = 56.7 / 32.2;
    let _floored = 2 / 3; // Results in 0
                         // remainder
    let _remainder = 43 % 5;
    // Boolean
    let _t = true;
    let _f: bool = false; // with explicit type annotation
                         // Character type
    let _c = 'z';
    let _z = 'ℤ';
    let _heart_eyed_cat = '😻';
    // Tuples
    let x: (i32, f64, u8) = (500, 6.4, 1);
    let _five_hundred = x.0;
    let _six_point_four = x.1;
    let _one = x.2;
    // Arrays
    let a = [1, 2, 3, 4, 5];
    let _first = a[0];
    let _second = a[1];
    // Functions
    another_function();
    // Function with parameter
    print_labeled_measurement(5, 'h');
    // Statement
    let _y = 6; // this is a statement
               // Expression
    let _y = {
        let x = 3;
        x + 1
    };
    // Functions with return types
    let _five = five();
    let _plus_one = plus_one(7);
    // Comments
    // This is comment
    /* This is a multiline comment
    This one too */
    // if expressions
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
    // else if expressions
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
    // Ternerary Operator
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {}", number);
    // Loop
    // loop {
        // println!("again!");
    // }
    // Returning values with loops
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {}", result);
    // while Loop
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }
    // for loop
    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("the value is: {}", element);
    }
}

fn another_function() {
    println!("Another function.");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {}{}", value, unit_label);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}