# Enums and Pattern Matching

### Defining an Enum

- Enums are a way of defining custom data types in a different way than you do with structs.

Say we need to work with IP addresses. Currently, two major standards are used for IP addresses: version four and version six. Any IP address can be either a version four or a version six address, but not both at the same time. That property of IP addresses makes the enum data structure appropriate, because an enum value can only be one of its variants. We can express this idea with the following piece of code:

```rust
enum IpAddrKind {
    V4,
    V6,
}
// IpAddrKind is now a custom type that can be used elsewhere in our code
```

### Enum Values

We can create instances of each of the two variants of `IpAddrKind` like this:

```rust
let four = IpAddrKind::V4;
let six = IpAddrKind::V6;
// Note that the variants of the enum are namespaced under its identifier, and we use a double colon to separate the two.
```

We can then for instance define a function that takes any of `IpAddrKind`:

```rust
fn route(ip_kind: IpAddrKind){}
// we can call this function either way:
fn main(){
	route(IpAddrKind::V4);
	route(IpAddrKind::V6);
}
```

We may be tempted to use Structs but let’s see how we can cut the lines of code we need to write the same program using enums:

```rust
		enum IpAddrKind {
        V4,
        V6,
    }

		struct IpAddr {
				// first field of type IpAddrKind
        kind: IpAddrKind,
				// second field of type String
        address: String,
    }
		// first instance home
    let home = IpAddr {
				// home is of kind V4
        kind: IpAddrKind::V4,
				// home is associated with a address of 127.0.0.1
        address: String::from("127.0.0.1"),
    };
		// second instance loopback
    let loopback = IpAddr {
				// loopback is of kind V6
        kind: IpAddrKind::V6,
				// loopback is of address ::1
        address: String::from("::1"),
    };
```

However we can represent the same concept using just an enum, which will create a more consice program: rather than putting an enum inside a struct, we can put data directly into each enum variant. Let’s make both `V4` and `V6` variants associate with the `String` values:

```rust
		enum IpAddr {
        V4(String),
        V6(String),
    // We attach data to each variant of the enum directly, so there is no need for an extra struct.
		}

    let home = IpAddr::V4(String::from("127.0.0.1"));

    let loopback = IpAddr::V6(String::from("::1"));
		
```

There is another advantage of using enums instead of structs, each variant can have different types and amounts of associated data. For instance `V4` will take always have four numeric components that will have values from 0 to 255. We can precise we only need to store values of type `u8` like so:

```rust
		enum IpAddr {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    let home = IpAddr::V4(127, 0, 0, 1);

    let loopback = IpAddr::V6(String::from("::1"));
```

There is one more similarity between enums and structs: just as we are able to define methods using `impl` with structs, we’re also able to define methods on enums. Here is an example:

```rust
fn main() {
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    impl Message {
        fn call(&self) {
            // method body would be defined here
        }
    }

    let m = Message::Write(String::from("hello"));
    m.call();
}
```

### The Option Enum and the Advantage Over Null Values

- The `Option` type encodes the very common scenario in which a value could be something or it could be nothing.
- Programming language design is often thought of in terms of which features you include, but the features you exclude are important too. Rust doesn’t have the null feature that many other languages have.

In his 2009 presentation “Null References: The Billion Dollar Mistake,” Tony Hoare, the inventor of null, has this to say:

![Untitled](Enums%20and%20%20d236b/Untitled.png)

- The problem with null values is that if you try to use a null value as a not-null value, you’ll get an error of some kind.
- However, the concept that null is trying to express is still a useful one: a null is a value that is currently invalid or absent for some reason.

Rust does not have nulls, but it does have an enum that can encode the concept of a value being present or absent. This enum is `Option<T>`, and it is [defined by the standard library](https://doc.rust-lang.org/std/option/enum.Option.html) as follows:

```rust
enum Option<T> {
    None,
    Some(T),
}
// The Option<T> enum is still just a regular enum, and Some(T) and None are still variants of type Option<T>.
/* <T> means the Some variant of the Option enum can hold one piece of data of any type, and that each concrete type
 that gets used in place of T makes the overall Option<T> type a different type. */
// Here is how we would create instances of this enum:
fn main() {
		// The type of some_number is Option<i32>
    let some_number = Some(5);
		// The type of some_string is Option<&str>
    let some_string = Some("a string");
		// Here, we tell Rust that we mean for absent_number to be of type Option<i32>.
    let absent_number: Option<i32> = None;
}
```

- When we have a `Some` value, we know that a value is present and the value is held within the `Some`.
- When we have a `None` value, in some sense, it means the same thing as null: we don’t have a valid value.
- This feature kind of reminds me of C++’s Class Templates

## The `match` Control Flow Construct

- Rust has an extremely powerful control flow construct called `match` that allows you to compare a value against a series of patterns and then execute code based on which pattern matches.
- The power of `match` comes from the expressiveness of the patterns and the fact that the compiler confirms that all possible cases are handled.

Think of a `match` expression as being like a coin-sorting machine: coins slide down a track with variously sized holes along it, and each coin falls through the first hole it encounters that it fits into.

![Untitled](Enums%20and%20%20d236b/Untitled%201.png)

Speaking of coins, let’s use them as an example for the `match` expression:

```rust
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
```

- This seems very similar to an expression used with `if`, but there’s a big difference: with `if`, the expression needs to return a Boolean value, but here, it can return any type. The type of `coin` in this example is the `Coin` enum that we defined on the first line.
- Next are the `match` arms. An arm has two parts: a pattern and some code. The first arm here has a pattern that is the value `Coin::Penny` and then the `=>` operator that separates the pattern and the code to run. The code in this case is just the value `1,`. Each arm is separated from the next with a comma.
- The resulting value of the expression in the matching arm is the value that gets returned for the entire `match`
 expression.

If you want to run multiple lines of code in a match arm, you must use curly brackets. For example:

```rust
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
				/* the following code prints “Lucky penny!” every time the method is called with a Coin::Penny
				but it still returns the last value of the block 1 */
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
```

### Patterns that Binds to Values

- Another neat feature of match expressions is they can bind to the parts of the values that match the pattern.

For example, for a few years the US issued quarters with different designs for each of the 50 states, no other coins, only quarters had these designs, we can add this information to our program like this:

```rust
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}
```

In the match expression we can print out to which state the quarter belongs to:

```rust
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn main() {
    value_in_cents(Coin::Quarter(UsState::Alaska));
}
```

### Matching with `Option<T>`

- We can also handle `Option<T>` using `match` as we did with the `Coin` enum!
- Let’s say we want to write a function that takes an `Option<i32>` and, if there’s a value inside, adds 1 to that value. If there isn’t a value inside, the function should retur the `None` value and not attempt to perform any operations.

The code would look like:

```rust
fn main() {
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }
		// The some(5) value doesn't match with the pattern None so it will continue to the next arm
    let five = Some(5);
		// the variable x in the body of plus_one will have the value some(5), it will go to None
    let six = plus_one(five);
		// In this case x is None, we enter the match and we will go to None
    let none = plus_one(None);
}
```

### Matches are Exhaustive

Consider the following function:

```rust
fn main() {
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}

» $ cargo run
   Compiling enums v0.1.0 (file:///projects/enums)
error[E0004]: non-exhaustive patterns: `None` not covered
   --> src/main.rs:3:15
    |
3   |         match x {
    |               ^ pattern `None` not covered
    |
    = help: ensure that all possible cases are being handled, possibly by adding wildcards or more match arms
    = note: the matched value is of type `Option<i32>`

For more information about this error, try `rustc --explain E0004`.
error: could not compile `enums` due to previous error
```

We didn’t handle the `None` case, so this code will cause a bug. It simply won’t compile if we call this function.

- Matches in Rust are *exhaustive*: we must exhaust every last possibility in order for the code to be safe.

### Catch-all Patterns and the _ Placeholder

Let’s implement a program that consists of a game where if you roll a 3 on a dice roll, your player doesn’t move, but instead gets a new fancy hat. If you roll a 7, your player loses a fancy hat. For all other values, your player moves that number of spaces on the game board.

```rust
// This is valid code
fn main() {
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
				// this last pattern will match all values not specifically listed
        other => move_player(other),
    }

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    fn move_player(num_spaces: u8) {}
}
```

- Rust also has a pattern we can use when we don’t want to use the value in the catch-all pattern: `_`. This tells Rust we aren’t going to use the value, so Rust won’t warn us about an unused variable.

Let’s change the rules of the game to be that if you roll anything other than a 3 or a 7, you must roll again

```rust
fn main() {
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => reroll(),
    }

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    fn reroll() {}
}
```

But if we change the rules and this time so that nothing else happens on your turn if you roll anything other than a 3 or a 7, we can express that by using the unit value.

```rust
fn main() {
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (),
    }

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
}
```

## Concise Control Flow with `if let`

- The `if let` syntax lets you combine `if` and `let` into a less verbose way to handle values that match one pattern while ignoring the rest.

Consider the following program:

```rust
fn main() {
    let config_max = Some(3u8);
    match config_max {
				// Only execute if the config_max value is the Some variant
        Some(max) => println!("The maximum is configured to be {}", max),
				// We don't want to anything with the None value
				// To satisfy the match expression, we have to add this which is annoying boilerplate code
        _ => (),
    }
}
```

Instead we could write this in a shorter way by using the `if let` expression:

```rust
fn main() {
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }
}
```

- The syntax `if let` takes a pattern and an expression separated by an equal sign. It works the same way as a `match`, where the expression is given to the `match` and the pattern is its first arm
- Using `if let` means less typing, less indentation, and less boilerplate code.
- We can include an `else` with an `if let`. The block of code that goes with the `else` is the same as the block of code that would go with the `_` case in the `match` expression.

Recall the Coin enum, let’s modify that code:

```rust
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    let coin = Coin::Penny;
    let mut count = 0;
    match coin {
        Coin::Quarter(state) => println!("State quarter from {:?}!", state),
        _ => count += 1,
    }
}
```

This could turn into:

```rust
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    let coin = Coin::Penny;
    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
}
```

- If you have a situation in which your program has logic that is too verbose to express using a `match`, remember that `if let` is in your Rust toolbox as well.

### Summary

- We’ve now covered how to use enums to create custom types that can be one of a set of enumerated values.
- We’ve shown how the standard library’s `Option<T>` type helps you use the type system to prevent errors.
- When enum values have data inside them, you can use `match` or `if let` to extract and use those values, depending on how many cases you need to handle.
- Your Rust programs can now express concepts in your domain using structs and enums.