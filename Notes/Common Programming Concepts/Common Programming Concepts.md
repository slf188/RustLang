# Common Programming Concepts

### Foundation

This chapter covers concepts that appear in almost every programming language and how they work in Rust. None of the concepts presented in this chapter are unique to Rust, but we’ll discuss them in the context of Rust.

We will learn about:

- Variables
- Basic Types
- Functions
- Comments
- Control Flow

**Note:** The Rust language has a a set of *keywords* that are reserved for use by the language only, much as in other languages. You can find a list of the keywords in [Appendix A](https://doc.rust-lang.org/book/appendix-01-keywords.html).

## Variables and Mutability

- By default variables are immutable. Rust gives us this functionality in order to let developers take advantage of the safety and easy concurrency Rust offers. However we still have the option to make variables mutable.
- When a variable is immutable, once a value is bound to a name, you can’t change that value.

```rust
fn main(){
	let x = 5;
	println!("The value of x is {}", x);
	x = 6;
	println!("The value of x is {}", x);
}

» 
$ cargo run
   Compiling variables v0.1.0 (file:///projects/variables)
error[E0384]: cannot assign twice to immutable variable `x`
 --> src/main.rs:4:5
  |
2 |     let x = 5;
  |         -
  |         |
  |         first assignment to `x`
  |         help: consider making this binding mutable: `mut x`
3 |     println!("The value of x is: {}", x);
4 |     x = 6;
  |     ^^^^^ cannot assign twice to immutable variable

For more information about this error, try `rustc --explain E0384`.
error: could not compile `variables` due to previous error
```

**Compiler errors can be frustrating, but really they only mean your program isn’t safely doing what you want it to do yet; they do *not* mean that you’re not a good programmer! Experienced Rustaceans still get compiler errors.**

The error message states `cannot assign twice to immutable variable x` because we tried to assign a second value to the immutable `x` variable.

- We can make variables mutable by adding the `mut` before the variable name.

```rust
fn main(){
	let mut x = 5;
	println!("The value of x is: {}", x);
	x = 6;
	println!("The value of x is: {}", x);
}

» The value of x is: 5
The value of x is: 6
```

### Constants

- C*onstants* are values that are bound to a name and are not allowed to change.
- You aren’t allowed to use `mut` with constants. Constants aren’t just immutable by default—they’re always immutable.
- You declare constants using the `const`keyword instead of the `let`keyword, and the type of the value *must* be annotated.

Here’s an example of a constant declaration:

```rust
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
```

- Rust’s naming convention for constants is to use all uppercase with underscores between words.
- Rust variables naming convention is to use underscores just like Python.

### Shadowing

- Shadowing means declaring a new variable with the same name as a previous variable.
- Rustaceans say that the first variable is *shadowed* by the second, which means that the second variable’s value is what the program sees when the variable is used.
- We can shadow a variable by using the same variable’s name and repeating the use of the `let` keyword as follows:

```rust
fn main() {
    let x = 5;
    let x = x + 1;
		// Inner scope
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }
    println!("The value of x is: {}", x);
}

» The value of x in the inner scope is: 12
The value of x is: 6
```

- The other difference between `mut` and shadowing is that because we’re effectively creating a new variable when we use the `let` keyword again, we can change the type of the value but reuse the same name.

For example, say our program asks a user to show how many spaces they want between some text by inputting space characters, and then we want to store that input as a number:

```rust
// The first spaces variable is a string type
let spaces = "   ";
// The second spaces variable is a number type
let spaces = spaces.len();
// Shadowing spares us from having to come up with different names
```

However, if we try to use the `mut` keyword for this we will get an error:

```rust
let mut spaces = "   ";
spaces = spaces.len();
» cargo run
   Compiling variables v0.1.0 (file:///projects/variables)
error[E0308]: mismatched types
 --> src/main.rs:3:14
  |
3 |     spaces = spaces.len();
  |              ^^^^^^^^^^^^ expected `&str`, found `usize`

For more information about this error, try `rustc --explain E0308`.
error: could not compile `variables` due to previous error
```

## Data Types

- Every value in Rust is of a certain *data type*, which tells Rust what kind of data is being specified so it knows how to work with that data.
- Keep in mind that Rust is a *statically typed* language, which means that it must know the types of all variables at compile time. So we must always add a type notation, like this:

```rust
let guess: u32 = "42".parse().expect("Not a number!");
```

If we don’t add the notation here, Rust will display an error, which means the compiler needs more information from us to know which type we want to use:

```rust
let guess = "42".parse().expect("Not a number!");

» $ cargo build
   Compiling no_type_annotations v0.1.0 (file:///projects/no_type_annotations)
error[E0282]: type annotations needed
 --> src/main.rs:2:9
  |
2 |     let guess = "42".parse().expect("Not a number!");
  |         ^^^^^ consider giving `guess` a type

For more information about this error, try `rustc --explain E0282`.
error: could not compile `no_type_annotations` due to previous error
```

We’ll look at two data type subsets: scalar and compound.

### Scalar Types

- A scalar type represents a single value.
- Rust has four primary scalar types:
    - Integers,
    - Floating-point numbers
    - Booleans
    - Characters

**Integer Types**

- An *integer* is a number without a fractional component.

Here is the table showing all the integer-types in Rust

![Screen Shot 2022-03-29 at 12.06.00.png](Common%20Pro%20ea10c/Screen_Shot_2022-03-29_at_12.06.00.png)

***Signed* and *unsigned* refer to whether it’s possible for the number to be negative—in other words, whether the number needs to have a sign with it (signed) or whether it will only ever be positive and can therefore be represented without a sign (unsigned).When the sign matters, a number is shown with a plus sign or a minus sign; however, when it’s safe to assume the number is positive, it’s shown with no sign.**

Here is the table showing all the integer literal in Rust

![Screen Shot 2022-03-29 at 12.11.56.png](Common%20Pro%20ea10c/Screen_Shot_2022-03-29_at_12.11.56.png)

*Integer Overflow:* Let’s say you have a variable of type `u8` that can hold values between 0 and 255. If you try to change the variable to a value outside of that range, such as 256, *integer overflow* will occur.

**Floating-point Types**

- Floating-point types are numbers with decimal points.
- The two primitive types for floating-points in Rust are `f32` and `f64` which are 32 and 64 bits, respectively. The default is `f64` because on modern CPUs it’s roughly the same speed as `f32` but is capable of more precision.
- `f64` has double precision and `f32` is a single precision float.

Here is an example:

```rust
fn main() {
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32
}
```

**Numeric Operations**

Rust supports the basic mathematical operations you’d expect for all of the number types: addition, subtraction, multiplication, division, and remainder.

- [Appendix B](https://doc.rust-lang.org/book/appendix-02-operators.html) contains a list of all operators that Rust provides.

Here is a demostration:

```rust
fn main() {
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let floored = 2 / 3; // Results in 0

    // remainder
    let remainder = 43 % 5;
}
```

**Boolean Type**

- Boolean type in Rust has two possible values: `true` and `false`. Booleans are one byte in size. For example:

```rust
fn main() {
    let t = true;
    let f: bool = false; // with explicit type annotation
}
```

**Character Type**

- Rust’s `char` type is the language’s most primitive alphabetic type
- Note that we specify `char` literals with single quotes, as opposed to string literals, which use double quotes.
- Rust’s `char` type is four bytes in size and represents a Unicode Scalar Value, which means it can represent a lot more than just ASCII. Accented letters; Chinese, Japanese, and Korean characters; emoji; and zero-width spaces are all valid `char` values in Rust.

```rust
fn main() {
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
}
```

### Compound Types

- *Compound types* can group multiple values into one type.
- Rust has two primitive compound types: tuples and arrays.

**Tuple Type**

- A tuple is a general way of grouping together a number of values with a variety of types into one compound type.
- Tuples have a fixed length: once declared, they cannot grow or shrink in size.
- We create a tuple by writing a comma-separated list of values inside parentheses.

```rust
let tup: (i32, f64, u8) = (500, 6.4, 1);
```

To get the individual values out of a tuple, we can use pattern matching to destructure a tuple value, like this:

```rust
fn main() {
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {}", y);
}
» The value of y is 6.4
```

We can also access a tuple element directly by using a period (`.`) followed by the index of the value we want to access. For example:

```rust
fn main() {
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;
}
```

**The Array Type**

- Unlike a tuple, every element of an array must have the same type.
- Unlike arrays in some other languages, arrays in Rust have a fixed length.

```rust
let a = [1, 2, 3, 4, 5];
```

- Arrays are useful when you want your data allocated on the stack rather than the heap.
- An array isn’t as flexible as the vector type, though. A vector is a similar collection type provided by the standard library that *is* allowed to grow or shrink in size.
- If you’re unsure whether to use an array or a vector, chances are you should use a vector.
- You write an array’s type using square brackets with the type of each element, a semicolon, and then the number of elements in the array, like so:

```rust
let a: [i32; 5] = [1, 2, 3, 4, 5];
// i32 indicates the type of each element
// 5 indicates the array contains 5 elements
```

- You can also initialize an array to contain the same value for each element by specifying the initial value, followed by a semicolon, and then the length of the array in square brackets, as shown here:

```rust
let a = [3; 5];
// This is the same as writing let a = [3, 3, 3, 3, 3];
```

- You can access elements of an array using indexing:

```rust
let a = [1, 2, 3, 4, 5];
let first = a[0];
let second = a[1];
```

## Functions

- Rust code uses *snake case* as the conventional style for function and variable names, in which all letters are lowercase and underscores separate words.

![Untitled](Common%20Pro%20ea10c/Untitled.png)

- We define a function in Rust by entering `fn` followed by a function name and a set of parentheses. The curly brackets tell the compiler where the function body begins and ends.
- Note that we defined `another_function` *after* the `main` function in the source code; we could have defined it before as well. Rust doesn’t care where you define your functions, only that they’re defined somewhere.

```rust
fn main() {
    println!("Hello, world!");

    another_function();
}

fn another_function() {
    println!("Another function.");
}
```

### Parameters

- We can define functions to have *parameters*, which are special variables that are part of a function’s signature.
- You can provide it with concrete values for those parameters. Technically, the concrete values are called *arguments*
- In function signatures, you *must* declare the type of each parameter.

```rust
fn main() {
    another_function(5);
}

fn another_function(x: i32) {
    println!("The value of x is: {}", x);
}

» The value of x is: 5
```

- When defining multiple parameters, separate the parameter declarations with commas.

```rust
fn main() {
    print_labeled_measurement(5, 'h');
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {}{}", value, unit_label);
}

» The measurement is: 5h
```

### Statements and Expressions

- Function bodies are made up of a series of statements optionally ending in an expression.
- *Statements* are instructions that perform some action and do not return a value. *Expressions* evaluate to a resulting value.
- In other languages such as C and Ruby, the assignment returns the value of the assignment, in those languages you can write `x = y = 6` and have both `x` and `y` have the value `6`; that is not the case in Rust.

Statement example:

```rust
fn main() {
    let y = 6; // this is a statement
}
```

Expression example:

```rust
fn main() {
    let y = {
        let x = 3;
        x + 1
    };
		/* The expression is
		{
	    let x = 3;
	    x + 1 <- no semicolon
		}
		*/
		// The expression block evaulates to 4
		// Expressions do not include ending semicolons
		// if we add a semicolon it turns into a statement
    println!("The value of y is: {}", y);
}

» The value of y is: 4
```

### Functions with Return Values

- Functions can return values to the code that calls them.
- We don’t name return values, but we must declare their type after an arrow (`->`).
- We don’t add a semicolon at the end of the return statement, if we do it will change from an expression to a statement.

Here is an example:

```rust
fn five() -> i32 {
    5
}

fn main() {
    let x = five();

    println!("The value of x is: {}", x);
}

» The value of x is: 5
```

Let’s look at another example:

```rust
fn main(){
	let x = plus_one(5);
	println!("The value of x is: {}", x);
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
» The value of x is: 6
```

## Comments

Here is a simple comment:

```rust
// hello, world
```

Comments can also be placed at the end of lines containing code

```rust
let lucky_number = 7; // I’m feeling lucky today
```

Here is a multiline comment:

```rust
/* A multiline 
comment */
```

## Control Flow

### `if` Expressions

- An `if` expression allows you to branch your code depending on conditions. You provide a condition and then state, “If this condition is met, run this block of code. If the condition is not met, do not run this block of code.”

```rust
fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
}

» condition was true
```

- Blocks of code associated with the conditions in `if` expressions are sometimes called *arms*, just like the arms in `match` expressions.
- Optionally, we can also include an `else`
 expression, which we chose to do here, to give the program an alternative block of code to execute should the condition evaluate to false.

It’s also worth noting that the condition in this code *must* be a `bool`. If the condition isn’t a `bool`, we’ll get an error. For example, try running the following code:

```rust
fn main() {
    let number = 3;

    if number {
        println!("number was three");
    }
}

» $ cargo run
   Compiling branches v0.1.0 (file:///projects/branches)
error[E0308]: mismatched types
 --> src/main.rs:4:8
  |
4 |     if number {
  |        ^^^^^^ expected `bool`, found integer

For more information about this error, try `rustc --explain E0308`.
error: could not compile `branches` due to previous error
```

The error indicates that Rust expected a `bool` but got an integer. Unlike languages such as Ruby and JavaScript, Rust will not automatically try to convert non-Boolean types to a Boolean.

### Handling Multiple Conditions with `else if`

You can use multiple conditions by combining `if` and `else` in an `else if` expression. For example:

```rust
fn main() {
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
}

» number is divisible by 3
```

Using too many `else if` expressions can clutter your code, so if you have more than one, you might want to refactor your code.

### U**sing `if` in a `let` Statement**

Because `if` is an expression, we can use it on the right side of a `let` statement to assign the outcome to a variable:

- This is what is known as ternerary operator in other languages.

```rust
fn main() {
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {}", number);

		// Or this could be rewritten as:
		/* if condition {
			number = 5;
		} else {
			number = 6;
		} */
}

» The value of number is: 5
```

In this program what we did is saying, if the variable `condition` is equal to true then we assign 5 to the `number` variable, otherwise we assign 6.

If the types are mismatched, as in the following example, we’ll get an error:

```rust
fn main() {
    let condition = true;

    let number = if condition { 5 } else { "six" };

    println!("The value of number is: {}", number);
}

» $ cargo run
   Compiling branches v0.1.0 (file:///projects/branches)
error[E0308]: `if` and `else` have incompatible types
 --> src/main.rs:4:44
  |
4 |     let number = if condition { 5 } else { "six" };
  |                                 -          ^^^^^ expected integer, found `&str`
  |                                 |
  |                                 expected because of this

For more information about this error, try `rustc --explain E0308`.
error: could not compile `branches` due to previous error
```

### Repetition with Loops

Rust has three kinds of loops: `loop`, `while`, and `for`. Let’s try each one

**`loop`**

- The `loop` keyword tells Rust to execute a block of code over and over again forever or until you explicitly tell it to stop.

```rust
fn main() {
    loop {
        println!("again!");
    }
		// This program will run continiously until we stop it manually
}

» again!
again!
again!
again!
^Cagain!
```

Rust also provides a way to break out of a loop using code. You can place the `break` keyword within the loop to tell the program when to stop executing the loop.

We can also use `continue`, which in a loop tells the program to skip over any remaining code in this iteration of the loop and go to the next iteration.

If we have loops within loops we can optionally specify a *loop label* on a loop that we can then use with `break` or `continue` to specify that those keywords apply to the labeled loop instead of the innermost loop. Here is an example:

```rust
fn main() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
								// This will interrupt the inner loop
                break;
            }
            if count == 2 {
								// This will interrupt the 'counting_up loop
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {}", count);
}

» count = 0
remaining = 10
remaining = 9
count = 1
remaining = 10
remaining = 9
count = 2
remaining = 10
End count = 2
```

**Returning values from loops**

We can return values using loopsand store those values inside other variables, like so:

```rust
fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);
}

» The result is 20
```

`**while` loop**

- A program will often need to evaluate a condition within a loop. While the condition is true, the loop runs. When the condition ceases to be true, the program calls `break`, stopping the loop.
- It’s possible to implement behavior like this using a combination of `loop`, `if`, `else`, and `break`; you could try that now in a program, if you’d like. However, this pattern is so common that Rust has a built-in language construct for it, called a `while` loop.

Here is a quick example:

```rust
fn main() {
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }
}

>> 3
2
1
```

`**for` loop**

We can use the `while` loop to traverse the elements of a collection, such as an array. For example:

```rust
fn main() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }
}

>> the value is 10
the value is: 20
the value is: 30
the value is: 40
the value is: 50
```

This is complete valid code, but it is error prone; we could cause the program to panic if the index value or test condition are incorrect. It’s also slow, because the compiler adds runtime code to perform the conditional check of whether the index is within the bounds of the array on every iteration through the loop.

- As a more concise alternative, you can use a `for` loop and execute some code for each item in a collection.

```rust
fn main() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {}", element);
    }
}
```

We will see the same output as the previous program, but the key difference is this program is more secure than the previous one because we’ve eliminated the chance of bugs that might result from going beyond the end of the array or not going far enough and missing some items.

### Summary

In this chapeter we learned about 

- Variables
- Scalar and Compound Data Types,
- Functions,
- Comments,
- `if`Expressions,
- Loops

Here is a program where all the topics get covered:

```rust
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
    let z = 2.0; // f64
    let k: f32 = 3.0; // f32
                      // addition
    let sum = 5 + 10;
    // subtraction
    let difference = 95.5 - 4.3;
    // multiplication
    let product = 4 * 30;
    // division
    let quotient = 56.7 / 32.2;
    let floored = 2 / 3; // Results in 0
                         // remainder
    let remainder = 43 % 5;
    // Boolean
    let t = true;
    let f: bool = false; // with explicit type annotation
                         // Character type
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
    // Tuples
    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;
    // Arrays
    let a = [1, 2, 3, 4, 5];
    let first = a[0];
    let second = a[1];
    // Functions
    another_function();
    // Function with parameter
    print_labeled_measurement(5, 'h');
    // Statement
    let y = 6; // this is a statement
               // Expression
    let y = {
        let x = 3;
        x + 1
    };
    // Functions with return types
    let five = five();
    let plus_one = plus_one(7);
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
    loop {
        println!("again!");
    }
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
```

When you’re ready to move on, we’ll talk about a concept in Rust that *doesn’t* commonly exist in other programming languages: **ownership**.