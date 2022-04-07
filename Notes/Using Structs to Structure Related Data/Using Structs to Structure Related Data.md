# Using Structs to Structure Related Data

- A struct, is a custom data type that lets you package together and name multiple related values that make up a meaningful group.
- If you’re familiar with an object-oriented language, a *struct* is like an object’s data attributes.
- In this chapter, we will compare and contrast structs with tuples and demonstrate when structs are a better way to organize data.
- We’ll demonstrate how to define and instantiate structs, how to define associated functions, specialy the ones that are known as methods, to specify certain behaviour associated with a struct type.

## Defining and Instantiating Structs

- Structs are similar to tuples, in that both hold multiple related values.
- Like tuples, the pieces of a struct can be different types. Unlike with tuples, in a struct you’ll name each piece of data so it’s clear what the values mean.
- Adding these names mean a struct is more flexible than a tuple: you don’t have to rely on the order of the data to specify or access the values of an instance.

To define a struct, we enter the `struct` keyword and name the entire struct, then inside the curly braces we define the names and the types of the pieces of data, which we call fields. For example:

```rust
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
```

To use a struct after we’ve defined it, we create an *instance* of that struct by specifying concrete values for each of the fields. We create an instance by stating the name of the struct and then add curly brackets containing `key: value` pairs, where the keys are the names of the fields and the values are the data we want to store in those fields. We don’t necesarily need to specify the fields in the same order declared in the struct. For example:

```rust
fn main() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
}
```

To get a specific value from the struct, we use the dot notation. If we wanted the email we could get it by writing `user1.email`. If this value is mutable then we can change a value by using dot notation and assigning a new value.

```rust
fn main() {
    let mut user1 = User { // note that the entire instance of the struct must be mutable
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");
}
```

Note that the entire instance must be mutable; Rust doesn’t allow us to mark only certain fields as mutables.

Let’s make a quick review of how we can return a struct instance in a function:

```rust
fn build_user(email: String, username: String) -> User {
    User {
        email: email, //repetitive
        username: username, // repetitive
        active: true,
        sign_in_count: 1,
    }
}
```

It makes sense to name the function parameters with the same name as the struct fields, but having to repeat the `email` and `username` field names and variables is a bit tedious. If the struct had more fields, repeating each name would get even more annoying. Luckily, there’s a convenient shorthand!

### Using the field init shorthand

Because the parameter names and the struct field names are exactly the same, we can use the *field init shorthand* syntax to rewrite `build_user` so that it behaves exactly the same but doesn’t have the repetition of `email` and `username`.

```rust
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
/* Because the email field and the email parameter have the same name,
 we only need to write email rather than email: email.
*/
}
```

### Creating Instances from other instances with struct update syntax

It’s often useful to create a new instance of a struct that includes most of the values from another instance, but changes some. You can do this using *struct update syntax*.

Here is how we would do so if we weren’t using the struct update syntax:

```rust
fn main() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };
}
```

Using the struct update syntax we can achieve the same behaviour with less lines of code. The syntax `..` specifies that the remaining fields not explicitly set should have the same value as the fields in the given instance:

```rust
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };
}
```

The `..user1` must come last to specify that any remaining fields should get their values from the corresponding fields in `user1`.

### Using tuple structs without named fields to create different types

- Rust also supports structs that look similar to tuples, called *tuple structs.*

To define a tuple struct, start with the `struct` keyword and the struct name followed by the types in the tuple. For example:

```rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}
```

Note that the `black` and `origin` values are different types, because they’re instances of different tuple structs. For example, a function that takes a parameter of type `Color` cannot take a `Point` as an argument, even though both types are made up of three `i32` values.

You can destructure them into their individual pieces, you can use a `.` followed by the index to access an individual value, and so on.

### Unit-like structs without any fields

You can also define structs that don’t have any fields! These are called *unit-like structs* because they behave similarly to `()`. Here is an example:

```rust
struct AlwaysEqual;

fn main() {
    let subject = AlwaysEqual;
}
```

To define `AlwaysEqual`, we use the `struct` keyword, the name we want, then a semicolon. No need for curly brackets or parentheses! Then we can get an instance of `AlwaysEqual` in the `subject` variable in a similar way: using the name we defined, without any curly brackets or parentheses.

## An Example program using Structs

Let’s calculate the area of a rectangle with structs. We’ll start by using variables and then refactor the program to use structs instead.

```rust
fn main() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

>> The area of the rectangle is 1500 square pixels.
```

The `area`function is supposed to calculate the area of one rectangle, but the function we wrote has two parameters, and it’s not clear anywhere in our program that the parameters are related.

### Refactoring with tuples

```rust
fn main() {
    let rect1 = (30, 50);

    println!(
        "The area of the rectangle is {} square pixels.",
        area(rect1)
    );
}

fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}
```

In one way, this program is better. Tuples let us add a bit of structure, and we’re now passing just one argument. But in another way, this version is less clear: tuples don’t name their elements, so we have to index into the parts of the tuple.

### Refactoring with structs: Adding more meaning

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
```

### Adding Useful Functionality with Derived Traits

It’d be useful to be able to print an instance of `Rectangle` while we’re debugging our program and see the values for all its fields.

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1 is {}", rect1);
}

>> error[E0277]: `Rectangle` doesn't implement `std::fmt::Display`
```

The `println!` macro can do many kinds of formatting, and by default, the curly brackets tell `println!` to use formatting known as `Display`.

The primitive types we’ve seen so far implement `Display` by default, because there’s only one way you’d want to show a `1` or any other primitive type to a user. But with structs, the way `println!` should format the output is less clear because there are more display possibilities: Do you want commas or not? Do you want to print the curly brackets? Should all the fields be shown? Due to this ambiguity, Rust doesn’t try to guess what we want, and structs don’t have a provided implementation of `Display`to use with `println!` and the `{}` placeholder.

If we continue reading the error we will see:

```rust
= help: the trait `std::fmt::Display` is not implemented for `Rectangle`
= note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
```

Let’s try it! The `println!` macro call will now look like `println!("rect1 is {:?}", rect1);`. Putting the specifier `:?` inside the curly brackets tells `println!` we want to use an output format called `Debug`. If we compile the program the output will be:

 

```rust
error[E0277]: `Rectangle` doesn't implement `Debug`
```

But again we get some helpful note:

```rust
= help: the trait `Debug` is not implemented for `Rectangle`
= note: add `#[derive(Debug)]` to `Rectangle` or manually `impl Debug for Rectangle`
```

Rust *does* include functionality to print out debugging information, but we have to explicitly opt in to make that functionality available for our struct. To do that, we add the outer attribute `#[derive(Debug)]` just before the struct definition.

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1 is {:?}", rect1);
}

>> rect1 is Rectangle { width: 30, height: 50 }
```

Nice! It’s not the prettiest output, but it shows the values of all the fields for this instance. When we have larger structs, it’s useful to have output that’s a bit easier to read; in those cases, we can use `{:#?}` instead of `{:?}` in the `println!` string.

Another way to print out a value using the `Debug` format is to use the `[dbg!` macro](https://doc.rust-lang.org/std/macro.dbg.html). Here is an example:

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect1);
}

>> [src/main.rs:10] 30 * scale = 60
[src/main.rs:14] &rect1 = Rectangle {
    width: 60,
    height: 50,
}
```

We can put `dbg!` around the expression `30 * scale` and, because `dbg!` returns ownership of the expression’s value, the `width` field will get the same value as if we didn’t have the `dbg!` call there.

Our `area` function is very specific: it only computes the area of rectangles. It would be helpful to tie this behavior more closely to our `Rectangle` struct, because it won’t work with any other type. Let’s look at how we can continue to refactor this code by turning the `area` function into an `area` *method* defined on our `Rectangle` type.

## Method Syntax

- Methods are similar to functions: we declare them with the `fn` keyword and a name, they can have parameters and a return value.
- Unlike functions, methods are defined within the context of a struct, and their first parameter is always `self`, which represents the instance of the struct the method is being called on.

### Defining Methods

Let’s define the area function within the Rectangle struct.

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}
```

- To define the function within the context of Rectangle, we start an `impl` (implementation) block for Rectangle.
- Everything within this `impl` block will be associated with the `Rectangle` type.
- Then we change the parameters to adopt the `self` keyword.
- To call the method we can use the method syntax to call the area method on our Rectangle instance.
- The `&self` is actually short for `self: &Self`. Within an `impl` block, the type `Self` is an alias for the type that the `impl` block is for.
- The main reason for using methods instead of functions is for organization purposes.

Note that we can choose to give a method the same name as one of the struct’s fields. For example:

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn width(&self) -> bool {
        self.width > 0
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }
}
```

![Screen Shot 2022-04-06 at 21.05.04.png](Using%20Stru%202d21b/Screen_Shot_2022-04-06_at_21.05.04.png)

### Methods with Parameters

Let’s add a method that takes more parameters this time, in this case we will make a method that returns true if the Rectangle we take as a parameter can fit completely within our Rectangle, otherwise it should return false:

```rust
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
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

» Can rect1 hold rect2? true
Can rect1 hold rect3? false
```

### Multiple `impl` Blocks

Each struct is allowed to have multiple `impl` blocks.

```rust
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
```

There’s no reason to separate these methods into multiple `impl` blocks here, but this is valid syntax. But we will see cases in which multiple `impl` blocks are useful.

### Summary

- By using structs, you can keep associated pieces of data connected to each other and name each piece to make your code clear.
- In `impl` blocks, you can define functions that are associated with your type, and methods are a kind of associated function that let you specify the behavior that instances of your structs have.