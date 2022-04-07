# Programming a Guessing Game

### Standard Library

To obtain user input and then print the result as output, we need to bring the `io`input/output library into scope. The `io` library comes from the standard library, known as `std`:

```rust
use std::io;
```

Rust has a few items defined in the standard library that it brings into the scope of every program. This set is called the *prelude*, and you can see everything in it [in the standard library documentation](https://doc.rust-lang.org/std/prelude/index.html).

### Main function

The `fn`syntax declares a new function, the parentheses, `()`, indicate there are no parameters, and the curly bracket, `{`, starts the body of the function.

### Storing values with variables

We use the `let` statement to create the variable.

```rust
let apples = 5
// This creates a variable named apples that binds it to the value 5
```

In Rust, variables are immutable by default.

To make a variable mutable, we add `mut` before the variable name:

```rust
let apples = 5; // immutable
let mut bananas = 5; // mutable
```

Calling `String::new` , returns a new instance of a `String`. This new function creates a new, empty string. You’ll find a `new` function on many types, because it’s a common name for a function that makes a new value of some kind.

In full, the `let mut guess = String::new();` line has created a mutable variable that will accept a new instance of a string.

### Receiving user input

To handle user input we’ll call the `stdin` function from the `io` module

```rust
io::stdin()
        .read_line(&mut guess)
```

If we hadn’t imported the `io` library with `use std::io` at the beginning of the program, we could still use the function by writing this function call as `std::io::stdin`

Next, the line `.read_line(&mut guess)` calls the `[read_line](https://doc.rust-lang.org/std/io/struct.Stdin.html#method.read_line)` method on the standard input handle to get input from the user. We’re also passing `&mut guess` as the argument to `read_line` to tell it what string to store the user input in.

The `&` indicates that this argument is a *reference*, which gives you a way to let multiple parts of your code access one piece of data without needing to copy that data into memory multiple times.

### Handling potential failure

```rust
.expect("Failed to read line");
```

We could have written this code as:

```rust
io::stdin().read_line(&mut guess).expect("Failed to read line");
```

However, one long line is difficult to read, so it’s best to divide it. It’s often wise to introduce a newline and other whitespace to help break up long lines when you call a method with the `.method_name()` syntax.

`read_line` puts whatever the user enters into the string we pass to it, but it also returns a value—in this case, an `[io::Result](https://doc.rust-lang.org/std/io/type.Result.html)`.

`Result`’s variants are `Ok`or `Err`. The `Ok`variant indicates the operation was successful, and inside `Ok` is the successfully generated value. The `Err` variant means the operation failed, and `Err` contains information about how or why the operation failed.

If this instance of `io::Result` is an `Err` value, `expect` will cause the program to crash and display the message that you passed as an argument to `expect`.

If this instance of `io::Result` is an `Ok` value, `expect` will take the return value that `Ok` is holding and return just that value to you so you can use it.

If you don’t call `expect`, the program will compile, but you’ll get a warning:

```rust
$ cargo build
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
warning: unused `Result` that must be used
  --> src/main.rs:10:5
   |
10 |     io::stdin().read_line(&mut guess);
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: `#[warn(unused_must_use)]` on by default
   = note: this `Result` may be an `Err` variant, which should be handled

warning: `guessing_game` (bin "guessing_game") generated 1 warning
    Finished dev [unoptimized + debuginfo] target(s) in 0.59s
```

Rust warns that you haven’t used the `Result` value returned from `read_line`, indicating that the program hasn’t handled a possible error.

### Printing values with `println!`

```rust
let x = 5;
let y = 10;
println!("x = {} and y = {}", x, y);
```

The `{}` set of curly brackets is a placeholder: think of `{}` as little crab pincers that hold a value in place.

## Using a crate to get more functionality

Rust doesn’t yet include random number functionality in its standard library. However, the Rust team does provide a `[rand` crate](https://crates.io/crates/rand) with said functionality.

### Adding new crates

Cargo’s coordination of external crates is where Cargo really shines. Before we can write code that uses `rand`, we need to modify the *Cargo.toml* file to include the `rand` crate as a dependency. Open that file now and add the following line to the bottom beneath the `[dependencies]` section header that Cargo created for you. Be sure to specify `rand` exactly as we have here, with this version number, or the code examples in this tutorial may not work.

```rust
rand = "0.8.3"
```

In `[dependencies]` you tell Cargo which external crates your project depends on and which versions of those crates you require.

Cargo understands [Semantic Versioning](http://semver.org/) (sometimes called *SemVer*), which is a standard for writing version numbers. The number `0.8.3` is actually shorthand for `^0.8.3`, which means any version that is at least `0.8.3` but below `0.9.0`.

After we’ve added this new crate to the dependencies section we can run:

```rust
$ cargo build
    Updating crates.io index
  Downloaded rand v0.8.3
  Downloaded libc v0.2.86
  Downloaded getrandom v0.2.2
  Downloaded cfg-if v1.0.0
  Downloaded ppv-lite86 v0.2.10
  Downloaded rand_chacha v0.3.0
  Downloaded rand_core v0.6.2
   Compiling rand_core v0.6.2
   Compiling libc v0.2.86
   Compiling getrandom v0.2.2
   Compiling cfg-if v1.0.0
   Compiling ppv-lite86 v0.2.10
   Compiling rand_chacha v0.3.0
   Compiling rand v0.8.3
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 2.53s
```

[Crates.io](https://crates.io/) is where people in the Rust ecosystem post their open source Rust projects for others to use.

If you immediately run `cargo build` again without making any changes, you won’t get any output aside from the `Finished` line. Cargo knows it has already downloaded and compiled the dependencies, and you haven’t changed anything about them in your *Cargo.toml* file.

### Ensuring reproducible builds with a Cargo.lock file

When you build a project for the first time, Cargo figures out all the versions of the dependencies that fit the criteria and then writes them to the *Cargo.lock* file. When you build your project in the future, Cargo will see that the *Cargo.lock* file exists and use the versions specified there rather than doing all the work of figuring out versions again.

### Updating a crate to get a new version

When you *do* want to update a crate, Cargo provides the command `update`, which will ignore the *Cargo.lock* file and figure out all the latest versions that fit your specifications in *Cargo.toml*.

You would see the following if you ran `cargo update`:

```rust
$ cargo update
    Updating crates.io index
    Updating rand v0.8.3 -> v0.8.4
```

### Generating a random number

First, we add the line `use rand::Rng`. The `Rng` trait defines methods that random number generators implement, and this trait must be in scope for us to use those methods.

In the first line, we call the `rand::thread_rng` function that gives us the particular random number generator that we’re going to use.

The `gen_range` method takes a range expression as an argument and generates a random number in the range. The kind of range expression we’re using here takes the form `start..end` and is inclusive on the lower bound but exclusive on the upper bound, so we need to specify `1..101` to request a number between 1 and 100. Alternatively, we could pass the range `1..=100`, which is equivalent.

**Note:** You won’t just know which traits to use and which methods and functions to call from a crate, so each crate has documentation with instructions for using it. Another neat feature of Cargo is that running the `cargo doc --open` command will build documentation provided by all of your dependencies locally and open it in your browser.

```rust
use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);

    println!("The secret number is: {}", secret_number);

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}
```

If we run this program a few times we will see the following:

```bash
$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 2.53s
     Running `target/debug/guessing_game`
Guess the number!
The secret number is: 7
Please input your guess.
4
You guessed: 4

$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/guessing_game`
Guess the number!
The secret number is: 83
Please input your guess.
5
You guessed: 5
```

### Comparing two numbers

First we add another `use` statement, bringing a type called `std::cmp::Ordering`

```rust
use std::cmp::Ordering;
```

The `Ordering` type is another enum and has the variants `Less`, `Greater`, and `Equal`. These are the three outcomes that are possible when you compare two values.

The `cmp` method compares two values and can be called on anything that can be compared. It takes a reference to whatever you want to compare with.

```rust
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("You guessed: {}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}
```

Note that this code won’t compile quite yet, as we will explain. If we try to build this program we will get the following error:

```bash
$ cargo build
   Compiling libc v0.2.86
   Compiling getrandom v0.2.2
   Compiling cfg-if v1.0.0
   Compiling ppv-lite86 v0.2.10
   Compiling rand_core v0.6.2
   Compiling rand_chacha v0.3.0
   Compiling rand v0.8.3
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
error[E0308]: mismatched types
  --> src/main.rs:22:21
   |
22 |     match guess.cmp(&secret_number) {
   |                     ^^^^^^^^^^^^^^ expected struct `String`, found integer
   |
   = note: expected reference `&String`
              found reference `&{integer}`

For more information about this error, try `rustc --explain E0308`.
error: could not compile `guessing_game` due to previous error
```

The core of the error states that there are *mismatched types*.

### Static Type System

Rust has a strong, static type system.

Rust was able to infer that `guess` should be a `String` and didn’t make us write the type. The `secret_number`, on the other hand, is a number type. A few of Rust’s number types can have a value between 1 and 100: `i32`, a 32-bit number; `u32`, an unsigned 32-bit number; `i64`, a 64-bit number; as well as others. Unless otherwise specified, Rust defaults to an `i32`

Rust cannot compare a string and a number type.

We want to convert the `String`the program reads as input into a real number type so we can compare it numerically to the secret number.

To convert guess from a string type to a i32 type we do the following:

```rust
	let guess: u32 = guess.trim().parse().expect("Please type a number!");
```

Rust allows us to *shadow* the previous value of `guess` with a new one. Shadowing lets us reuse the `guess` variable name rather than forcing us to create two unique variables, such as `guess_str` and `guess` for example.

We bind this new variable to the expression `guess.trim().parse()`. The `guess` in the expression refers to the original `guess` variable that contained the input as a string. The `trim` method on a `String` instance will eliminate any whitespace at the beginning and end, which we must do to be able to compare the string to the `u32`, which can only contain numerical data.

The `[parse` method on strings](https://doc.rust-lang.org/std/primitive.str.html#method.parse) parses a string into some kind of number. Because this method can parse a variety of number types, we need to tell Rust the exact number type we want by using `let guess: u32`. The colon (`:`) after `guess` tells Rust we’ll annotate the variable’s type. Rust has a few built-in number types; the `u32` seen here is an unsigned, 32-bit integer. It’s a good default choice for a small positive number.

The `parse` method will only work on characters that can logically be converted into numbers and so can easily cause errors. If, for example, the string contained `A👍%`, there would be no way to convert that to a number. Because it might fail, the `parse` method returns a `Result` type, much as the `read_line` method does (discussed earlier in [“Handling Potential Failure with the `Result` Type”](https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html#handling-potential-failure-with-the-result-type)).

If we run the program now we will get:

```bash
$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 0.43s
     Running `target/debug/guessing_game`
Guess the number!
The secret number is: 58
Please input your guess.
  76
You guessed: 76
Too big!
```

### Allowing Multiple Guesses with Looping

The `loop` keyword creates an infinite loop. We’ll add a loop to give users more chances at guessing the number:

```rust

    println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => println!("You win!"),
        }
		}
```

The user could always interrupt the program by using the keyboard shortcut ctrl-c. But there’s another way to escape this program by writting `quit`

### Quitting after a correct guess

Let’s program the game to quit when the user wins by adding a `break` statement:

```rust
match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
```

Adding the `break` line after `You win!` makes the program exit the loop when the user guesses the secret number correctly. Exiting the loop also means exiting the program, because the loop is the last part of `main`.

### Handling invalid input

To further refine the game’s behavior, rather than crashing the program when the user inputs a non-number, let’s make the game ignore a non-number so the user can continue guessing. We can do that by altering the line where `guess` is converted from a `String` to a `u32`.

```rust
let guess: u32 = match guess.trim().parse() {
    Ok(num) => num,
    Err(_) => continue,
};
```

Remember that `parse` returns a `Result` type and `Result` is an enum that has the variants `Ok` or `Err`. We’re using a `match` expression here, as we did with the `Ordering` result of the `cmp` method.

### Complete program

We should comment out the line that prints what the secret number is, so the final program looks like this:

```rust
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..101);
    // println!("The secret number is {}", secret_number);
    loop {
        println!("Please input your guess");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("You guessed {}", guess);
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
```

### Summary

This project was a hands-on way to introduce you to many new Rust concepts: `let`, `match`, functions, the use of external crates, and more.