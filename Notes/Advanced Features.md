# Advanced Features

- By now, we have learned the most important part of Rust, use this chapter as a reference if we encounter unknowns in Rust.
- These features are very specific.
- We will cover:
    - Unsafe Rus
    - Advanced Traits
    - Advanced Types
    - Advanced functions and closures
    - Macros

## Unsafe Rust

- All the code we’ve discussed so far has had Rust’s memory safety guarantees enforced at compile time.
- However, Rust has a second language hidden inside it that doesn’t enforce these memory safety guarantees: it’s called *unsafe Rust*
- It exists because although the code *might* be okay, if the Rust compiler doesn’t have enough information to be confident, it will reject the code. In these cases, you can use unsafe code to tell the compiler, “Trust me, I know what I’m doing.”
- The downside of unsafe Rust is we will use it a our own risk, problems in our code can occur.
- Another reason Rust has an unsafe alter ego is that the underlying computer hardware is inherently unsafe. If we didn’t have this feature, we could not be able to do certain tasks in low level code.
- Things such as interacting directly with the os or writting our own os

### Unsafe superpowers

- To switch to unsafe Rust, use the `unsafe` keyword and then start a new block that holds the unsafe code.
- What unsafe will allow you to do is:
    - Dereference a raw pointer
    - Call an unsafe function or method
    - Access or modify a mutable static variable
    - Implement an unsafe trait
    - Access fields of `union`s

Let’s look at the five things unsafe rust will allow us to do:

### Dereference a raw pointer

- Unsafe Rust has two new types called *raw pointers* that are similar to references.
- Raw pointers can be immutable or mutable and are written as `*const T` and `*mut T`, respectively.
- Different from references and smart pointers, raw pointers:
    - Allowed to ignore the borrowing rules
    - Aren’t guaranteed to point to valid memory
    - Are allowed to be null
    - Don’t implement any automatic cleanup

Here is an example:

```rust
fn main() {
    let mut num = 5;
		// we use as to cast an immutable and mutable reference into their correspoding raw pointer types
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;
}
```

Here we show how to create a raw pointer at a particular location:

```rust
fn main() {
    let address = 0x012345usize;
    let r = address as *const i32;
}
```

Recall that we can create raw pointers in safe code, but we can’t *dereference* raw pointers and read the data being pointed to:

```rust
fn main() {
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }
}
```

Creating a pointer does no harm; it’s only when we try to access the value that it points at that we might end up dealing with an invalid value.

### Calling an unsafe function or method

- Unsafe functions and methods look exactly like regular functions and methods, but they have an extra `unsafe`
 before the rest of the definition.
- By calling an unsafe function within an `unsafe` block, we’re saying that we’ve read this function’s documentation and take responsibility for upholding the function’s contracts.

```rust
fn main() {
    unsafe fn dangerous() {}

    unsafe {
        dangerous();
    }
}
```

### Creating safe abstractions over unsafe code

- Just because a function contains unsafe code doesn’t mean we need to mark the entire function as unsafe.
- In fact, wrapping unsafe code in a safe function is a common abstraction.

Consider `split_at_mut`:

```rust
fn main() {
    let mut v = vec![1, 2, 3, 4, 5, 6];

    let r = &mut v[..];

    let (a, b) = r.split_at_mut(3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);
}
```

If we try to implement the function using safe code we would get the following:

```rust
fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();

    assert!(mid <= len);

    (&mut values[..mid], &mut values[mid..])
}

fn main() {
    let mut vector = vec![1, 2, 3, 4, 5, 6];
    let (left, right) = split_at_mut(&mut vector, 3);
}

$ cargo run
   Compiling unsafe-example v0.1.0 (file:///projects/unsafe-example)
error[E0499]: cannot borrow `*values` as mutable more than once at a time
 --> src/main.rs:6:31
  |
1 | fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
  |                         - let's call the lifetime of this reference `'1`
...
6 |     (&mut values[..mid], &mut values[mid..])
  |     --------------------------^^^^^^--------
  |     |     |                   |
  |     |     |                   second mutable borrow occurs here
  |     |     first mutable borrow occurs here
  |     returning this value requires that `*values` is borrowed for `'1`

For more information about this error, try `rustc --explain E0499`.
error: could not compile `unsafe-example` due to previous error
```

The way to make this compoile would be:

```rust
use std::slice;

fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    let ptr = values.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

fn main() {
    let mut vector = vec![1, 2, 3, 4, 5, 6];
    let (left, right) = split_at_mut(&mut vector, 3);
}
```

### Using extern functions to call external code

- Sometimes, your Rust code might need to interact with code written in another language. For this, Rust has a keyword, `extern`, that facilitates the creation and use of a *Foreign Function Interface (FFI)*.
- An FFI is a way for a programming language to define functions and enable a different (foreign) programming language to call those functions.

Here is an example of how to set up an integration with the `abs` function from the C standard library. Functions declared within `extern` blocks are always unsafe to call from Rust code.

```rust
extern "C" {
    fn abs(input: i32) -> i32;
}

fn main() {
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
}

$ cargo run
Absolute value of -3 according to C: 3
```

<aside>
🗺️ **Calling rust functions from other languages:** We can also use `extern` to create an interface that allows other languages to call Rust functions. We also need to add a `#[no_mangle]` annotation to tell the Rust compiler not to mangle the name of this function. *Mangling* is when a compiler changes the name we’ve given a function to a different name that contains more information for other parts of the compilation process to consume but is less human readable. **Every programming language compiler mangles names slightly differently, so for a Rust function to be nameable by other languages, we must disable the Rust compiler’s name mangling.**

</aside>

```rust
#![allow(unused)]
fn main() {
#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}
}
```

### Accesing or modifying a mutable static variable

- In Rust, global variables are called *static* variables.
- The names of static variables are in `SCREAMING_SNAKE_CASE` by convention.
- Another difference between constants and static variables is that static variables can be mutable.

```rust
static HELLO_WORLD: &str = "Hello, world!";

fn main() {
    println!("name is: {}", HELLO_WORLD);
}
```

### Implementing an unsafe trait

A trait is unsafe when at least one of its methods has some invariant that the compiler can’t verify. We can declare that a trait is `unsafe` by adding the `unsafe` keyword before `trait`

```rust
unsafe trait Foo {
    // methods go here
}

unsafe impl Foo for i32 {
    // method implementations go here
}

fn main() {}
```

### Accessing fields of a union

A `union` is similar to a `struct`, but only one declared field is used in a particular instance at one time. Unions are primarily used to interface with unions in C code.

### When to use unsafe code

When you have a reason to use `unsafe` code, you can do so, and having the explicit `unsafe` annotation makes it easier to track down the source of problems when they occur.

## Advanced Traits

### Specifying placeholder types in trait definitions with associated types

- *Associated types* connect a type placeholder with a trait such that the trait method definitions can use these placeholder types in their signatures.
- One example of a trait with an associated type is the `Iterator` trait that the standard library provides.

```rust
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}
```

Let’s examine the difference between the two concepts with an example that implements the `Iterator` trait on a `Counter` struct. This implementation specifies the `Item` type is `u32`:

```rust
impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        // --snip--
```

### Default generic type parameters and operator overloading

- When we use generic type parameters, we can specify a default concrete type for the generic type. This eliminates the need for implementors of the trait to specify a concrete type if the default type works.
- The syntax for specifying a default type for a generic type is `<PlaceholderType=ConcreteType>` when declaring the generic type.
- A great example of a situation where this technique is useful is with operator overloading.
- Rust doesn’t allow you to create your own operators or overload arbitrary operators. But you can overload the operations and corresponding traits listed in `std::ops` by implementing the traits associated with the operator.

```rust
use std::ops::Add;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn main() {
    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );
}
```

The `add` method adds the `x` values of two `Point` instances and the `y` values of two `Point` instances to create a new `Point`.

The default generic type in this code is:

```rust
trait Add<Rhs=Self> {
    type Output;

    fn add(self, rhs: Rhs) -> Self::Output;
}
```

### Fully qualified syntax for disambiguation: calling methods with the same name

- Nothing in Rust prevents a trait from having a method with the same name as another trait’s method, nor does Rust prevent you from implementing both traits on one type.
- When calling methods with the same name, you’ll need to tell Rust which one you want to use.

```rust
trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

fn main() {
    let person = Human;
    person.fly();
}

$ cargo run
*waving arms furiously*
```

### Using supertraits to require one trait’s functionality within another trait

Sometimes, you might need one trait to use another trait’s functionality. In this case, you need to rely on the dependent trait also being implemented. The trait you rely on is a *supertrait* of the trait you’re implementing.

For example if we use the outlineprint:

```rust
use std::fmt;

trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

$ cargo run
**********
*        *
* (1, 3) *
*        *
**********
```

## Advanced Types

### Using the newtype pattern for type safety and abstraction

- The newtype pattern is useful for tasks beyond those we’ve discussed so far, including statically enforcing that values are never confused and indicating the units of a value.
- Another use of the newtype pattern is in abstracting away some implementation details of a type: the new type can expose a public API that is different from the API of the private inner type.

### Creating type synonyms with type aliases

Rust provides the ability to declare a *type alias* to give an existing type another name. For this we use the `type` keyword. For example, we can create the alias `Kilometers` to `i32` like so:

```rust
fn main() {
    type Kilometers = i32;

    let x: i32 = 5;
    let y: Kilometers = 5;

    println!("x + y = {}", x + y);
}

$ cargo run
x + y = 10
```

The main use case for type synonyms is to reduce repetition. For example:

```rust
Box<dyn Fn() + Send + 'static>
```

Writting this lenghty program would be kind of rough to write:

```rust
fn main() {
    let f: Box<dyn Fn() + Send + 'static> = Box::new(|| println!("hi"));

    fn takes_long_type(f: Box<dyn Fn() + Send + 'static>) {
        // --snip--
    }

    fn returns_long_type() -> Box<dyn Fn() + Send + 'static> {
        // --snip--
        Box::new(|| ())
    }
}
```

Instead we could shorten it up using type aliases

```rust
fn main() {
    type Thunk = Box<dyn Fn() + Send + 'static>;

    let f: Thunk = Box::new(|| println!("hi"));

    fn takes_long_type(f: Thunk) {
        // --snip--
    }

    fn returns_long_type() -> Thunk {
        // --snip--
        Box::new(|| ())
    }
}
```

### The never type that never returns

Rust has a special type named `!` that’s known in type theory lingo as the *empty type* because it has no values.

```rust
fn bar() -> ! {
	// Functions that return never are called diverging functions.
}
```

## Advanced functions and closures

### Function pointers

- We’ve talked about how to pass closures to functions; you can also pass regular functions to functions!
- Doing this with function pointers will allow you to use functions as arguments to other functions.

```rust
fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn main() {
    let answer = do_twice(add_one, 5);

    println!("The answer is: {}", answer);
}

$ cargo run
The answer is: 12
```

### Returning closures

- Closures are represented by traits, which means you can’t return closures directly.
- In most cases where you might want to return a trait, you can instead use the concrete type that implements the trait as the return value of the function.

## Macros

- We’ve used macros like `println!` throughout this book, but we haven’t fully explored what a macro is and how it works.
- The term macro refers to a family of features in Rust: declarative macros with macro_rules! and three kinds of procedural macros:
    - Custom #[derive] macros that specify code added with the derive attribute used on structs and enums
    - Attribute-like macros that define custom attributes usable on any item
    - Function-like macros that look like function calls but operate on the tokens specified as their argument

### The diff between macros and functions

- Macros are a way of writing code that writes other code, which is known as metaprogramming.
- Metaprogramming is useful for reducing the amount of code you have to write and maintain, which is also one of the roles of functions.
- A function signature must declare the number and type of parameters the function has. Macros, on the other hand, can take a variable number of parameters: we can call println!("hello") with one argument or println!("hello {}", name) with two arguments.
- The downside to implementing a macro instead of a function is that macro definitions are more complex than function definitions because you’re writing Rust code that writes Rust code.
- Due to this indirection, macro definitions are generally more difficult to read, understand, and maintain than function definitions.
- Another important difference between macros and functions is that you must define macros or bring them into scope before you call them in a file, as opposed to functions you can define anywhere and call anywhere.

### Declarative Macros with macro_rules! for General Metaprogramming

- The most widely used form of macros in Rust is declarative macros. These are also sometimes referred to as “macros by example,” “macro_rules! macros,” or just plain “macros.”

To define a macro, you use the macro_rules! construct. For example:

```rust
let v: Vec<u32> = vec![1, 2, 3];
```

We could also use the vec! macro to make a vector of two integers or a vector of five string slices.

Here is a simplified definition of vec!

```rust
// macro will be available anywhere
#[macro_export]
// macro_rules means macro definition along with the name of the macro
macro_rules! vec {
		// arm pattern
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}
```

Once we call the macro above we can then do:

```rust
{
    let mut temp_vec = Vec::new();
    temp_vec.push(1);
    temp_vec.push(2);
    temp_vec.push(3);
    temp_vec
}
```

### Procedural macros for generating code from attributes

- Procedural macros accept some code as an input, operate on that code, and produce some code as an output
- The three kinds of procedural macros (custom derive, attribute-like, and function-like) all work in a similar fashion.
- When creating procedural macros, the definitions must reside in their own crate with a special crate type.

```rust
use proc_macro;

#[some_attribute]
pub fn some_name(input: TokenStream) -> TokenStream {
}
```

### How to write a custom derive macro

Let’s create a crate named hello_macro that defines a trait named HelloMacro with one associated function named hello_macro. The default implementation will print Hello, Macro! My name is TypeName! where TypeName is the name of the type on which this trait has been defined.

```rust
use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

#[derive(HelloMacro)]
struct Pancakes;

fn main() {
    Pancakes::hello_macro();
}

$ cargo run
Hello Macro! My name is Pancakes!
```

The first step is to make a new library crate, like this:

```rust
$ cargo new hello_macro --lib
```

Next, we’ll define the `HelloMacro` trait and its associated function:

```rust
pub trait HelloMacro {
    fn hello_macro();
}
```

We have a trait and its function. At this point, our crate user could implement the trait to achieve the desired functionality, like so:

```rust
use hello_macro::HelloMacro;

struct Pancakes;

impl HelloMacro for Pancakes {
    fn hello_macro() {
        println!("Hello, Macro! My name is Pancakes!");
    }
}

fn main() {
    Pancakes::hello_macro();
}
```

The next step is to define the procedural macro.Let’s start a new crate called hello_macro_derive inside our hello_macro project:

```rust
$ cargo new hello_macro_derive --lib
```

We need to declare the hello_macro_derive crate as a procedural macro crate. We’ll also need functionality from the syn and quote crates, as you’ll see in a moment, so we need to add them as dependencies. Add the following to the Cargo.toml file for hello_macro_derive:

```rust
[lib]
proc-macro = true

[dependencies]
syn = "1.0"
quote = "1.0"
```

To start defining the procedural macro, place the code in Listing 19-31 into your src/lib.rs file for the hello_macro_derive crate.

```rust
use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_hello_macro(&ast)
}
```

Now that we have the code to turn the annotated Rust code from a TokenStream into a DeriveInput instance, let’s generate the code that implements the HelloMacro trait on the annotated type

```rust
fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                println!("Hello, Macro! My name is {}!", stringify!(#name));
            }
        }
    };
    gen.into()
}
```

### Attribute-like macros

- Attribute-like macros are similar to custom derive macros, but instead of generating code for the derive attribute, they allow you to create new attributes.
- They’re also more flexible: derive only works for structs and enums; attributes can be applied to other items as well, such as functions.

Here’s an example of using an attribute-like macro:

```rust
#[route(GET, "/")]
fn index() {
```

The signature of the macro definition function would look like this:

```rust
#[proc_macro_attribute]
pub fn route(attr: TokenStream, item: TokenStream) -> TokenStream {
```

### Function like macros

- Function-like macros define macros that look like function calls.
- Function-like macros take a TokenStream parameter and their definition manipulates that TokenStream using Rust code as the other two types of procedural macros do. An example of a function-like macro is an sql! macro that might be called like so:

```rust
let sql = sql!(SELECT * FROM posts WHERE id=1);
```

This macro would parse the SQL statement inside it and check that it’s syntactically correct, which is much more complex processing than a macro_rules! macro can do. The sql! macro would be defined like this:

```rust
#[proc_macro]
pub fn sql(input: TokenStream) -> TokenStream {
```

## Summary

We’ve introduced several complex topics so that when you encounter them in error message suggestions or in other peoples’ code, you’ll be able to recognize these concepts and syntax.