# Patterns and Matching

- Patterns are a special syntax in Rust for matching against the structure of types, both complex and simple.
- Using patterns in conjunction with `match` expressions and other constructs gives you more control over a program’s control flow.
- A pattern consists of some combination of the following:
    - Literals
    - Variables
    - Wildcards
    - Placeholders
    - Destructured arrays, enums, structs or tuples
- These components describe the shape of the data we’ll be working on.
- Recall the coin sorting machine, if the value fits the shape of the pattern we can use the named pieces, if it doesn’t fit the code won’t run.

## All the places patterns can be used

### match Arms

Formally speaking, `match` expressions are defined as the keyword `match`, a value to match on, and one or more match arms that consist of a pattern and an expression to run if the value matches that arm’s pattern, like this:

```rust
match VALUE {
    PATTERN => EXPRESSION,
    PATTERN => EXPRESSION,
    PATTERN => EXPRESSION,
}
```

- A requirement for match expressions is they need to be exhaustive i.e. in the sense all possibilities must be accounted for.
- A way to ensure this is the case is to have a catchall pattern for the last arm. The `_` pattern can be useful when you want to ignore any value not specified, for example.

### Conditional if let expressions

- `if let` expressions are a shorter way to write the equivalent of a `match` expression that only matches with one case.
- It’s also possible to to mix and match `if let`, `else if`, and `else if let` expressions. Doing so gives us more flexibility than `match` expressions.

```rust
fn main() {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using your favorite color, {}, as the background", color);
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }
}

$ cargo run
Using purple as the background color
```

The downside of using `if let` expressions is that the compiler doesn’t check exhaustiveness, whereas with `match` expressions it does.

### while let Conditional Loops

- The `while let` conditional loop allows a `while` loop to run for as long as a pattern continues to match.

As an example, we show while let loop that uses a vector as a stack and prints the values in the vector in the opposite order:

```rust
fn main() {
    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }
}

$ cargo run
3
2
1
```

### for Loops

- The for loop is the most common loop construction in Rust code.
- In a `for` loop, the pattern is the value that directly follows the keyword `for`, so in `for x in y` the `x` is the pattern.

Here we show how we can destructure or break apart a tuple using a for loop

```rust
fn main() {
    let v = vec!['a', 'b', 'c'];

    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }
}

$ cargo run
   Compiling patterns v0.1.0 (file:///projects/patterns)
    Finished dev [unoptimized + debuginfo] target(s) in 0.52s
     Running `target/debug/patterns`
a is at index 0
b is at index 1
c is at index 2
```

We use the `enumerate` method to adapt an iterator to produce a value and that value’s index in the iterator, placed into a tuple. 

- The first value is (0, ‘a’)

### let Statements

More formally, let statements look like this:

```rust
let PATTERN = EXPRESSION;
```

Prior to this chapter, we’ve used patterns in other places as well, including in `let` statements. For example:

```rust
let x = 5;
```

In this statement Rust takes x as a pattern that means “bind what matches here to the variable `x`.” Because x is the whole pattern, this pattern effectively means “bind everything to the variable `x`, whatever the value is.”

- To see the pattern matching aspect of `let` consider the following:

```rust
let (x, y, z) = (1, 2, 3);
```

Here, we match a tuple against a pattern. Rust compares the value `(1, 2, 3)` to the pattern `(x, y, z)` and sees that the value matches the pattern, so Rust binds `1` to `x`, `2` to `y`, and `3` to `z`.

- If the number of elements in the pattern doesn’t match the number of elements in the tuple, the overall type won’t match and we’ll get a compiler error.

Example:

```rust
let (x, y) = (1, 2, 3);
```

If we compile this we will get:

```rust
$ cargo run
   Compiling patterns v0.1.0 (file:///projects/patterns)
error[E0308]: mismatched types
 --> src/main.rs:2:9
  |
2 |     let (x, y) = (1, 2, 3);
  |         ^^^^^^ expected a tuple with 3 elements, found one with 2 elements
  |
  = note: expected tuple `({integer}, {integer}, {integer})`
             found tuple `(_, _)`

For more information about this error, try `rustc --explain E0308`.
error: could not compile `patterns` due to previous error
```

If we wanted to ignore one or more of the values in the tuple we could’ve used `_` or `..` 

### Function Parameters

Function parameters can also be patterns:

```rust
fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({}, {})", x, y);
}

fn main() {
    let point = (3, 5);
    print_coordinates(&point);
}

$ cargo run
Current location: (3, 5)
```

## Refutability: Whether a pattern might failt to match

- Patterns come in two forms: refutable and irrefutable.
- Patterns that will match for any possible value passed are *irrefutable*.
- An example of irrefutable could be `let x = 5;` because x matches anything and therefore nothing can fail.
- Patterns that can fail to match for some possible value are *refutable*.
- An example could be `Some(x)` in the expression `if let Some(x) = a_value` because the a_value variable is None rather than Some.
- Function parameters, `let` statements, and `for` loops can only accept irrefutable patterns, because the program cannot do anything meaningful when values don’t match.
- In general, you shouldn’t have to worry about the distinction between refutable and irrefutable patterns; however, you do need to be familiar with the concept of refutability so you can respond when you see it in an error message.

Let’s take the following example:

```rust
let Some(x) = some_option_value; // if some_option_value was None it would fail the Some(x) pattern

$ cargo run
   Compiling patterns v0.1.0 (file:///projects/patterns)
error[E0005]: refutable pattern in local binding: `None` not covered
   --> src/main.rs:3:9
    |
3   |     let Some(x) = some_option_value;
    |         ^^^^^^^ pattern `None` not covered
    |
    = note: `let` bindings require an "irrefutable pattern", like a `struct` or an `enum` with only one variant
    = note: for more information, visit https://doc.rust-lang.org/book/ch18-02-refutability.html
    = note: the matched value is of type `Option<i32>`
help: you might want to use `if let` to ignore the variant that isn't matched
    |
3   |     if let Some(x) = some_option_value { /* */ }
    |

For more information about this error, try `rustc --explain E0005`.
error: could not compile `patterns` due to previous error
```

So to handle this we could instead use `if let`

```rust
if let Some(x) = some_option_value {
    println!("{}", x);
}
```

But if we give `if let` a pattern that will always match, such as `x`, as shown here, the compiler will give a warning.

```rust
		if let x = 5 {
        println!("{}", x);
    };

// Rust complains that it doesn’t make sense to use if let with an irrefutable pattern:
$ cargo run
   Compiling patterns v0.1.0 (file:///projects/patterns)
warning: irrefutable `if let` pattern
 --> src/main.rs:2:8
  |
2 |     if let x = 5 {
  |        ^^^^^^^^^
  |
  = note: `#[warn(irrefutable_let_patterns)]` on by default
  = note: this pattern will always match, so the `if let` is useless
  = help: consider replacing the `if let` with a `let`

warning: `patterns` (bin "patterns") generated 1 warning
    Finished dev [unoptimized + debuginfo] target(s) in 0.39s
     Running `target/debug/patterns`
5
```

## Pattern Syntax

### Matching Literals

You can match patterns against literals directly. The following code gives some examples:

```rust
fn main() {
    let x = 1;

    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
}

$ cargo run
one
```

### Matching Named Variables

Named variables are irrefutable patterns that match any value, and we’ve used them many times in the book.

```rust
fn main() {
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {:?}", y),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {:?}", x, y);
}

$ cargo run
Matched, y = 5
at the end: x = Some(5), y = 10
```

### Multiple Patterns

In `match` expressions, you can match multiple patterns using the `|` syntax, which means *or:*

```rust
fn main() {
    let x = 1;

    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
}

$ cargo run
one or two
```

### Matching ranges of values with ..=

The `..=` syntax allows us to match to an inclusive range of values:

```rust
fn main() {
    let x = 5;

    match x {
				// 1, 2, 3, 4, 5 will match this first arm
				// specifying the range is much shorter than using |
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }
}
```

Ranges are only allowed with numeric values or char values:

```rust
fn main() {
    let x = 'c';

    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }
}

$ cargo run
early ASCII letter
```

### Destructuring to break apart values

We can also use patterns to destructure structs, enums, and tuples to use different parts of these values. Let’s walk through each value.

**Destructuring Structs**

Here we have a `Point` struct with two fields, `x` and `y`, that we can break apart using a pattern with a `let` statement.

```rust
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point { x: 0, y: 7 };

    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);
}
```

Or either way we could use:

```rust
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point { x: 0, y: 7 };

    let Point { x, y } = p;
    assert_eq!(0, x);
    assert_eq!(7, y);
}
```

Here is an example of a match expression with a destructured struct:

```rust
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point { x: 0, y: 7 };

    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        Point { x: 0, y } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }
}

$ cargo run
On the y axis at 7
```

**Destructuring Enums**

As an example we could use the following:

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    let msg = Message::ChangeColor(0, 160, 255);

    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.")
        }
        Message::Move { x, y } => {
            println!(
                "Move in the x direction {} and in the y direction {}",
                x, y
            );
        }
        Message::Write(text) => println!("Text message: {}", text),
        Message::ChangeColor(r, g, b) => println!(
            "Change the color to red {}, green {}, and blue {}",
            r, g, b
        ),
    }
}

$ cargo run
Change the color to red 0, green 160, and blue 255
```

**Destructuring nested enums and structs**

Matching can work on nested items too! For example:

```rust
enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}

fn main() {
    let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));

    match msg {
        Message::ChangeColor(Color::Rgb(r, g, b)) => println!(
            "Change the color to red {}, green {}, and blue {}",
            r, g, b
        ),
        Message::ChangeColor(Color::Hsv(h, s, v)) => println!(
            "Change the color to hue {}, saturation {}, and value {}",
            h, s, v
        ),
        _ => (),
    }
}

$ cargo run
Change the color to hue 0, saturation 160, and value 255
```

**Destructuring tuples and structs**

We can mix, match, and nest destructuring patterns in even more complex ways. The following example shows a complicated destructure where we nest structs and tuples inside a tuple and destructure all the primitive values out:
``

```rust
fn main() {
    struct Point {
        x: i32,
        y: i32,
    }

    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });
}
```

Destructuring with patterns is a convenient way to use pieces of values, such as the value from each field in a struct, separately from each other.

### Ignoring values in a pattern

There are a few ways to ignore entire values or parts of values in a pattern: using the `_` pattern (which you’ve seen), using the `_` pattern within another pattern, using a name that starts with an underscore, or using `..` to ignore remaining parts of a value.

**Ignoring an entire value using _**

We’ve used the underscore (`_`) as a wildcard pattern that will match any value but not bind to the value:

```rust
fn foo(_: i32, y: i32) {
    println!("This code only uses the y parameter: {}", y);
}

fn main() {
    foo(3, 4);
}

$ cargo run
This code only uses the y parameter: 4
```

**Ignoring parts of a value with a nested _**

We can also use `_` inside another pattern to ignore just part of a value:

```rust
fn main() {
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }

    println!("setting is {:?}", setting_value);
}

$ cargo run
Can't overwrite an existing customized value
setting is Some(5)
```

In all other cases (if either `setting_value` or `new_setting_value` are `None`) expressed by the `_` pattern in the second arm, we want to allow `new_setting_value` to become `setting_value`.

```rust
fn main() {
    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, _, third, _, fifth) => {
            println!("Some numbers: {}, {}, {}", first, third, fifth)
        }
    }
}

$ cargo run
Some numbers: 2, 8, 32
```

**Ignoring unused variable by starting its name with _**

If you create a variable but don’t use it anywhere, Rust will usually issue a warning because that could be a bug. But sometimes it’s useful to create a variable you won’t use yet, such as when you’re prototyping or just starting a project. In this situation, you can tell Rust not to warn you about the unused variable by starting the name of the variable with an underscore.

```rust
fn main() {
    let _x = 5;
    let y = 10;
}
```

**Ignoring remaining parts of a value with ..**

With values that have many parts, we can use the `..` syntax to use only a few parts and ignore the rest, avoiding the need to list underscores for each ignored value.

```rust
fn main() {
    struct Point {
        x: i32,
        y: i32,
        z: i32,
    }

    let origin = Point { x: 0, y: 0, z: 0 };

    match origin {
        Point { x, .. } => println!("x is {}", x),
    }
}

x is 0
```

The syntax `..` will expand to as many values as it needs to be. Here is the example of a tuple:

```rust
fn main() {
    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, .., last) => {
            println!("Some numbers: {}, {}", first, last);
        }
    }
}

$ cargo run
Some numbers: 2, 32
```

### Extra conditionals with match guards

A *match guard* is an additional `if` condition specified after the pattern in a `match` arm that must also match, along with the pattern matching, for that arm to be chosen.

These are useful for expressing more complex ideas that a pattern alone allows.

```rust
fn main() {
    let num = Some(4);

    match num {
        Some(x) if x % 2 == 0 => println!("The number {} is even", x),
        Some(x) => println!("The number {} is odd", x),
        None => (),
    }
}

$ cargo run
The number 4 is even
```

### @ Bindings

The *at* operator (`@`) lets us create a variable that holds a value at the same time we’re testing that value to see whether it matches apattern.

```rust
fn main() {
    enum Message {
        Hello { id: i32 },
    }

    let msg = Message::Hello { id: 5 };

    match msg {
        Message::Hello {
            id: id_variable @ 3..=7,
        } => println!("Found an id in range: {}", id_variable),
        Message::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }
        Message::Hello { id } => println!("Found some other id: {}", id),
    }
}

$ cargo run
Found an id in range: 5
```

Using `@` lets us test a value and save it in a variable within one pattern.

## Summary

- Rust’s patterns are very useful in that they help distinguish between different kinds of data.
- When used in `match` expressions, Rust ensures your patterns cover every possible value, or your program won’t compile.
- Patterns in `let` statements and function parameters make those constructs more useful, enabling the destructuring of values into smaller parts at the same time as assigning to variables.
- We can create simple or complex patterns to suit our needs.