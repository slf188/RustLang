# Generic Types, Traits and Lifetimes

- Every programming language has tools for effectivey handling the duplication of concepts. In Rust on such tool is *generics*.
- Generics are abstract stand-ins for concrete types or other properties. Similar to the way a function takes parameters with unknown values to run the same code on multiple concrete values, like `i32` or `String`.
- This kind of reminds me to [C++ Function Template](https://www.programiz.com/cpp-programming/function-template).  We will focus on creating a generic function from two functions that differ only in the types of their parameters. We will also explain how to do so with structs and enum definitions.
- Finally we will discuss *lifetimes*, which are generics that give the compiler information about how references relate to each other.

### Removing Duplication by Extracting a Function

Consider a short program that finds the largest number in a list:

```rust
fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
		// set the largest number to 34
    let mut largest = number_list[0];
		// iterate through the number list, replace the value of largest if the current number is greater than itself
    for number in number_list {
        if number > largest {
						// 34 > 34
						// 50 > 34
						// 25 > 50
						// 100 > 50
						// 65 > 100
            largest = number;
        }
    }

    println!("The largest number is {}", largest);
    assert_eq!(largest, 100);
}

>> The largest number is 100
```

To find the largest number in two different lists of numbers, we can duplicate the code and use the same logic at two different places.

```rust
fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let mut largest = number_list[0];

    for number in number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {}", largest);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let mut largest = number_list[0];

    for number in number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {}", largest);
}

>> The largest number is 100
The largest number is 6000
```

Although this code works fine, there is a lot of repetition in it, so we in order to avoid this code which is tedious and error prone, we can create a function that operates on any list of integers given to it in a parameter. This type of function will therefore create clearer code and express the concept of finding the largest number in a list abstractly.

This is the way:

```rust
// the function has a parameter list, it represents any concrete slice of i32 values that we might pass into the function
fn largest(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);
    assert_eq!(result, 100);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let result = largest(&number_list);
    println!("The largest number is {}", result);
    assert_eq!(result, 6000);
}

>> The largest number is 100
The largest number is 6000
```

In sum, here are the steps we took to change the code:

1. Identify duplicate code
2. Extract duplicate code into the body of the function and specify the inputs and return values of that code
3. Update the two instances of duplicated code to call the function instead.

Next, we’ll focus on how to use generics to operate on abstract types. For example say we had two functions: one that finds the largest item in a slice of `i32` values and one that finds the largest item in a slice of `char` values. How would we eliminate that duplication? Let’s find out!

## Generic Data Types

- We can use generics to create definitions for items like functions, enums, methods or structs, which we can use with many different data types.
- Here is the [difference between a method and a function](https://stackoverflow.com/a/155655)

### In function definitions

- When defining a function that uses generics, we place the generics in the signature of the function where we would usually specify the data types of the parameters and return value.
- Doing so makes our code more flexible and provides more functionality to callers of our function while preventing code duplication.

The following example shows two functions that both find the largest element in a slice:

```rust
// finds the largest i32 element in a slice
fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}
// finds the largest char element in a slice
fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_i32(&number_list);
    println!("The largest number is {}", result);
    assert_eq!(result, 100);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_char(&char_list);
    println!("The largest char is {}", result);
    assert_eq!(result, 'y');
}

>> The largest number is 100
The largest char is y
```

- The function bodies have the same code, so let’s eliminate the duplication by introducing a generic type parameter in a single function.

To do so we need to do the following steps:

1. We need to name the type parameter. We’ll use `T` by convention.
2. Rust’s type-naming convention is CamelCase.
3. To define the generic `largest` function, place type name declarations inside angle brackets, `<>`, between the name of the function and the parameter list, like this:

```rust
// this function has one parameter named list, which is a slice of type T, it will return a value of the same type T
fn largest<T>(list: &[T]) -> T {
```

So let’s illustrate how we can use this generic function and call it for both lists of `i32` and `char` values. Note this program won’t compile yet...

```rust
fn largest<T>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
}

>> $ cargo run
   Compiling chapter10 v0.1.0 (file:///projects/chapter10)
error[E0369]: binary operation `>` cannot be applied to type `T`
 --> src/main.rs:5:17
  |
5 |         if item > largest {
  |            ---- ^ ------- T
  |            |
  |            T
  |
help: consider restricting type parameter `T`
  |
1 | fn largest<T: **std::cmp::PartialOrd>**(list: &[T]) -> T {
  |             ++++++++++++++++++++++

For more information about this error, try `rustc --explain E0369`.
error: could not compile `chapter10` due to previous error
```

- The note mentions `std::cmp::PartialOrd`, which is a *trait*. We’ll talk about traits in the next section. This error states that the body of `largest` won’t work for all possible types that `T` could be. Because we want to compare values of type T in the body, we need to enable comparisons, `std::cmp::PartialOrd`.

### In Struct Definitions

- We can also define structs to use a generic type parameter in one or more fields using the `<>` syntax.

Here is an example of a struct that can hold x and y coordinate values of any type:

```rust
// The syntax for structs is, declaring the name of the struct and a T type inside angle brackets after the name of the struct
struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
}
```

If we create an instance of Point<T> that has values of different types it won’t compile:

```rust
struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let wont_work = Point { x: 5, y: 4.0 };
}

>> $ cargo run
   Compiling chapter10 v0.1.0 (file:///projects/chapter10)
error[E0308]: mismatched types
 --> src/main.rs:7:38
  |
7 |     let wont_work = Point { x: 5, y: 4.0 };
  |                                      ^^^ expected integer, found floating-point number

For more information about this error, try `rustc --explain E0308`.
error: could not compile `chapter10` due to previous error
```

To allow a behaviour where Struct can take x and y for different types, we can use change the definition of `Point` to be generic over types `T` and `U` where `x` is of type `T` and `y` is of type `U`:

```rust
struct Point<T, U> {
    x: T,
    y: U,
}

fn main() {
    let both_integer = Point { x: 5, y: 10 };
    let both_float = Point { x: 1.0, y: 4.0 };
    let integer_and_float = Point { x: 5, y: 4.0 };
		// this code will compile
}
```

### In Enum Definitions

- As we did with structs, we can also define enums to hold generic data types in their variants.

Here is a quick example:

```rust
// Option<T> is an enum that is generic over type T
enum Option<T> {
		// Option<T> has two variants
    Some(T), // Some holds one value of type T
    None, // None variant that doesn't hold any value
}
```

Enums can use multiple generic types as well, here is another example:

```rust
enum Result<T, E> {
    // Result<T, E> is generic over two types and two variants
		// It would be nice to use this enum in a case where an operation might succeed or fail
		// Example: We used this enum to open a file
		Ok(T),
    Err(E),
}
```

### In Method Definitions

- We can implement methods on structs and enums and use generic types in their definitions too.

```rust
struct Point<T> {
    x: T,
    y: T,
}
// declare T after impl, by doing so Rust identifies that the type in Point is a generic type
impl<T> Point<T> {
		// method x on Point<T> that returns a reference to the data in field x
    fn x(&self) -> &T {
        &self.x
    }
}

fn main() {
    let p = Point { x: 5, y: 10 };

    println!("p.x = {}", p.x());
}
```

Another option would be to implement the same method but this time for `Option<f32>` types rather than `Option<T>`

```rust
struct Point<T> {
    x: T,
    y: T,
}
// another instance where T is not of type f32
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}
// type Point<32> will have a method name distance_from_origin
impl Point<f32> {
		// this method measure how far our point is from the point at coordinates (0.0, 0.0)
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn main() {
    let p = Point { x: 5, y: 10 };

    println!("p.x = {}", p.x());
}
```

Generic type parameters in a struct definition aren’t always the same as those you use in that struct’s method signatures:

```rust
struct Point<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> Point<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
		// this instance has i32 for x and f64 for y
    let p1 = Point { x: 5, y: 10.4 };
		// string slice for x and char for y
    let p2 = Point { x: "Hello", y: 'c' };
		// Calling mixup on p1 with argument p2 gives us p3
    let p3 = p1.mixup(p2); // we will have i32 for x and char for y

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}

» p3.x = 5, p3.y = c
```

The purpose of this example is to show a situation in which some generic parameters are declared with `impl` and some are declared with the method definition.

### Performance of Code using Generics

- So the question now is, **is there a runtime cost when using generic type parameters? The good news is that Rust implements generics in such a way that your code doesn’t run any slower using generic types than it would with concrete types.**
- Rust accomplishes this by performing monomorphization of the code that is using generics at compile time. *Monomorphization* is the process of turning generic code into specific code by filling in the concrete types that are used when compiled.
- When a compiler handles *Monomorphization* the compiler does the opposite of the steps we used to create generic functions, the compiler looks at all the places where generic code is called and generates code for the concrete types the generic code is called with.

Here is a demostration of how monomorphization works:

```rust
// When Rust compiles this code, it performs monomorphization.
let integer = Some(5); // The compiler identifies Option<T> instance of type i32
let float = Some(5.0); // The compiler identifies Option<T> instance of type f64
```

**Before Monomorphization:**

```rust
enum Option<T>{
	Some(T),
	None,
}

fn main(){
}
```

**After Monomorphization:**

```rust
enum Option_i32 {
    Some(i32),
    None,
}

enum Option_f64 {
    Some(f64),
    None,
}

fn main() {
    let integer = Option_i32::Some(5);
    let float = Option_f64::Some(5.0);
}
```

**When the code runs, it performs just as it would if we had duplicated each definition by hand. The process of monomorphization makes Rust’s generics extremely efficient at runtime.**

## Traits: Defining Shared Behaviour - Check Twice

- A *trait* tells the Rust compiler about functionality a particular type has and can share with other types. We can use it to define shared behaviour.
- Traits are very similar to a feature called Interfaces in other programming languages, although it has some differences.

C# supports Interfaces in the following manner:

```csharp
using System;
namespace CsharpInterface {

  interface IPolygon {
		// all methods of an interface are fully abstract(method without body)
    // method without body
    void calculateArea(int l, int b);

  }

  class Rectangle : IPolygon {

    // implementation of methods inside interface
    public void calculateArea(int l, int b) {

      int area = l * b;
      Console.WriteLine("Area of Rectangle: " + area);
    }
  }

  class Program {
    static void Main (string [] args) {

      Rectangle r1 = new Rectangle();
    
      r1.calculateArea(100, 200);

    }
  }
}

» Area of Rectangle: 20000
```

### Defining a trait

- A type’s behaviour consists of the methods we can call on that type.
- For example, say we have two structs, `NewsArticle` which holdsa news story filed in a particular location and `Tweet`

that can have at most 280 characters, along with data that indicates if it was a new tweet, retweet or a reply.

```rust
pub trait Summary {
    fn summarize(&self) -> String;
}
```

- We declare a trait using the `trait` keyword and then the traits name, we can specify if the trait is `pub` so the crates using this crate can make use of it too. Inside the trait we declare the method signatures that describe the behaviour of the types that implement this trait.
- After the method signature within the trait, instead of providing an implementation inside curly braces, we use a semicolon.
- A trait can have multiple methods in its body: the methods must be listed one per line and each line shall end with a semicolon.

### Implementing a Trait on a Type

Let’s see how we implement the `Summary` trait on the `NewsArticle` struct that handles 4 attributes, headline, author and location to create the return value of `summarize`. For the `Tweet` struct, we define `summarize` as the username followed by the entire text of the tweet.

```rust
pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
```

- Implementing a trait on a type is similar to implementing regular methods. The difference is that after `impl`, we put the trait name that we want to implement, then use the `for` keyword, and then specify the name of the type we want to implement the trait for.
- Within the `impl` block instead of adding a semicolon after each signature, we use curly brackets and fill in the method body with the specific behavior that we want the methods of the trait to have for the particular type.
- Now the library has implemented the `Summary` trait on `NewsArticle` and `Tweet`

Here is an example of how a binary crate could use our `agregator` library crate:

```rust
use aggregator::{Summary, Tweet};

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());
}

» 1 new tweet: horse_ebooks: of course, as you probably already know, people
```

### Default Implementations

- Sometimes it’s useful to have default behavior for some or all of the methods in a trait instead of requiring implementations for all methods on every type.

Here is an example of how we can specify a default string for the `summarize` method:

```rust
pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}
// We are no longer defining the summarize method for NewsArticle
impl Summary for NewsArticle {}

fn main() {
    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };

    println!("New article available! {}", article.summarize());
}

» New article available! (Read more...)
```

Creating a default implementation for `summarize` doesn’t require us to change anything about the implementation of `Summary` on `Tweet`.

Default implementations can call other methods in the same trait, even if those other methods don’t have a default implementation. For example, we could define the `Summary` trait to have a `summarize_author` method whose implementation is required, and then define a `summarize` method that has a default implementation that calls the `summarize_author` method:

```rust
pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}
```

To use this version of `Summary` we only need to define `summarize_author` when we implement it:

```rust
impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}
```

After defining the `summarize_author` method we can now do the following:

```rust
fn main(){
	let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());
}

» 1 new tweet: (Read more from @horse_ebooks...)
```

### Traits as parameters

Now let’s see how we can use traits to define functions that accept many different types.

For example in the following program, we implement the `Summary` trait on the `NewsArticle` and `Tweet` types. We can define a `notify` function that calls the `summarize` method on its `item` parameter, to do so we can do the following:

```rust
pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
```

We can call `notify` and pass in any instance of `NewsArticle` or `Tweet`.

**Trait Bound Syntax**

The `impl Trait` syntax works for straightforward cases but is actually syntax sugar for a longer form, which is called a *trait bound*; it looks like this:

```rust
// this longer form is equivalent to the previous example, but it is more verbose
pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}
```

If we have two parameters that implement `Summary`. Using the `impl Trait` syntax looks like this:

```rust
pub fn notify(item1: &impl Summary, item2: &impl Summary) {
```

If we wanted to allow `item1` and `item2` to have different types, we can do so with:

```rust
pub fn notify<T: Summary>(item1: &T, item2: &T) {
```

**Specifying Multiple Trait Bounds with the + Syntax**

We can specify if the `notify` method that `item` can implement both `Display` and `Summary`:

```rust
pub fn notify(item: &(impl Summary + Display)) {
```

The `+` syntax is also valid with trait bounds on generic types:

```rust
pub fn notify<T: Summary + Display>(item: &T) {
```

**Clearer Trait Bounds with where Clauses**

Using too many trait bounds has its downsides. Functions with multiple generic type parameters can make the function signature hard to read. For this reason Rust has a solution, `where`, instead of writting this:

```rust
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
```

We can write this:

```rust
fn some_function<T, U>(t: &T, u: &U) -> i32
    where T: Display + Clone,
          U: Clone + Debug
{
```

### Returning Types that Implement Traits

We can also use the `impl Trait` syntax in the return position to return a value of some type that implements a trait, as shown here:

```rust
// In this case, returns_summarizable returns a Tweet
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    }
}
```

By using `impl Summary`  for the return type, we specify that the `returns_summarizable` function returns some type that implements the `Summary` trait without naming the concrete type.

- This technique is specially useful in the context of closures and iterators.

However, we can only use `impl Trait` if we are returning a single type, this following program wouldn’t work because it either returns a `Tweet` or a `NewsArticle`:

```rust
// this isn’t allowed due to restrictions around how the impl Trait syntax is implemented in the compiler
fn returns_summarizable(switch: bool) -> impl Summary {
    if switch {
        NewsArticle {
            headline: String::from(
                "Penguins win the Stanley Cup Championship!",
            ),
            location: String::from("Pittsburgh, PA, USA"),
            author: String::from("Iceburgh"),
            content: String::from(
                "The Pittsburgh Penguins once again are the best \
                 hockey team in the NHL.",
            ),
        }
    } else {
        Tweet {
            username: String::from("horse_ebooks"),
            content: String::from(
                "of course, as you probably already know, people",
            ),
            reply: false,
            retweet: false,
        }
    }
}
```

### Fixing the Largest Function with Trait Bounds

Let’s write a function that compares two values of type `T` using the greater than operator.

```rust
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
}

>> $ cargo run
   Compiling chapter10 v0.1.0 (file:///projects/chapter10)
error[E0369]: binary operation `>` cannot be applied to type `T`
 --> src/main.rs:5:17
  |
5 |         if item > largest {
  |            ---- ^ ------- T
  |            |
  |            T
  |
help: consider restricting type parameter `T`
  |
1 | fn largest<T: **std::cmp::PartialOrd>**(list: &[T]) -> T {
  |             ++++++++++++++++++++++

For more information about this error, try `rustc --explain E0369`.
error: could not compile `chapter10` due to previous error
```

Because that operator is defined as a default method on the standard library trait `std::cmp::PartialOrd`, we need to specify `PartialOrd` in the trait bounds for `T` so we can proceed onto using the `>` operator.

The function name shall be modified to look like this:

```rust
fn largest<T: PartialOrd>(list: &[T]) -> T {

>> $ cargo run
   Compiling chapter10 v0.1.0 (file:///projects/chapter10)
error[E0508]: **cannot move out of type `[T]`**, a non-copy slice
 --> src/main.rs:2:23
  |
2 |     let mut largest = list[0];
  |                       ^^^^^^^
  |                       |
  |                       cannot move out of here
  |                       move occurs because `list[_]` has type `T`, which does not implement the `Copy` trait
  |                       help: consider borrowing here: `&list[0]`

error[E0507]: cannot move out of a shared reference
 --> src/main.rs:4:18
  |
4 |     for &item in list {
  |         -----    ^^^^
  |         ||
  |         |data moved here
  |         |move occurs because `item` has type `T`, which does not implement the `Copy` trait
  |         help: consider removing the `&`: `item`

Some errors have detailed explanations: E0507, E0508.
For more information about an error, try `rustc --explain E0507`.
error: could not compile `chapter10` due to 2 previous errors
```

- The key line in this error is “cannot move out of type `[T]`". In this function we are essentially finding the largest element in `i32` and `char` types. The reason for this error is because types like `i32` and `char` have known size that is stored in the stack, so they implement the `Copy` trait. But in the `largest` function the  `list` parameter doesnt’ implement the `Copy` trait.

To implement the `Copy` trait, we must add `Copy` to the trait bounds of `T`:

```rust
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
}

>> The largest number is 100
The largest char is y
```

If we don’t want to restrict the `largest` function to the types that implement the `Copy` trait, we could specify that `T` has the trait bound `Clone` instead of `Copy`. Then we could clone each value in the slice when we want the `largest` function to have ownership.

### Using Trait bounds to Conditionally Implement Methods

Let’s take a look at the following example, the type `Pair<T>` always implements the `new` function to return a new instance of `Pair<T>` recall that `Self`
 is a type alias for the type of the `impl` block, which in this case is `Pair<T>`.

```rust
use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
		// this will return Pair<T>
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}
```

We can also conditionally implement a trait for any type that implements another trait. This technique is called *blanket implementations* and it is extensively used within the Rust standard library. Here is a quick example:

```rust
impl<T: Display> ToString for T {
    // --snip--
}
// we call this in the following way:
fn main(){
	let s = 3.to_string();
}
```

Traits and trait bounds let us write code that uses generic type parameters to reduce duplication but also specify to the compiler that we want the generic type to have particular behavior. In dynamically typed languages, we would get an error at runtime if we called a method on a type which didn’t define the method. But Rust moves these errors to compile time so we’re forced to fix the problems before our code is even able to run.

## Validating References with Lifetimes

- Every reference in Rust has a *lifetime,* which is the period where the reference is valid.
- Annotating lifetimes is not even a concept most other programming languages have, so this is going to feel unfamiliar.

### Preventing dangling references with lifetimes

The main aim of lifetimes is to prevent dangling references, which cause a program to reference data other than the data it’s intended to reference.

Consider the following example:

```rust
fn main() {
    {
				// here we declare a variable without giving an initial value
        let r;
				// the variable exists in outer scope
        {
						// value of r is referenced to x
            let x = 5;
            r = &x;
        }
				// this code won't compile because the value r is referring to has gone out of scope before we try to use it
        println!("r: {}", r);
    }
}

>> $ cargo run
   Compiling chapter10 v0.1.0 (file:///projects/chapter10)
error[E0597]: `x` does not live long enough
  --> src/main.rs:7:17
   |
7  |             r = &x;
   |                 ^^ borrowed value does not live long enough
8  |         }
   |         - `x` dropped here while still borrowed
9  | 
10 |         println!("r: {}", r);
   |                           - borrow later used here

For more information about this error, try `rustc --explain E0597`.
error: could not compile `chapter10` due to previous error
```

- `x` doesn’t “live long enough.” `x` will be out of scope when the inner scope ends. But `r` is still valid for the outer scope; because its scope is larger, we say that it “lives longer.” But how does Rust determine this code will be invalid? It uses a borrow checker.

### Borrow Checker

- The Rust compiler has a *borrow checker* that compares scopes to determine whether all borrows are valid.

```rust
fn main() {
    {
        let r;                // ---------+-- 'a
                              //          |
        {                     //          |
            let x = 5;        // -+-- 'b  |
            r = &x;           //  |       |
        }                     // -+       |
                              //          |
        println!("r: {}", r); //          |
    }                         // ---------+
}

// we’ve annotated the lifetime of r with 'a and the lifetime of x with 'b
// the inner 'b block is much smaller than the outer 'a lifetime block
// At compile time, Rust compares the size of the two lifetimes and sees that because 'b is shorter than 'a: the subject of the reference doesn’t live as long as the reference
```

We can’t fix and handle the error by avoiding a dangling reference by doing the following:

```rust
fn main() {
    {
        let x = 5;            // ----------+-- 'b
                              //           |
        let r = &x;           // --+-- 'a  |
                              //   |       |
        println!("r: {}", r); //   |       |
                              // --+       |
    }                         // ----------+
}
// Here, x has the lifetime 'b, which in this case is larger than 'a. This means r can reference x because Rust knows that the reference in r will always be valid while x is valid.
```

Now that we know how Rust analyzes lifetimes to ensure references will be valid, let us understand generic lifetimes of parameters and return values in the context of functions.

### Generic Lifetimes in Functions

- Let’s write a function that returns the longer of two string slices. This function will take two string slices and return a string slice.

```rust
fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";
    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}

>> The longest string is abcd
```

- Note that we want the function to take string slices, which are references, because we don’t want the `longest` function to take ownership of its parameters.

If we implement the longest function it won’t compile:

```rust
fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";
    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}

fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

>> $ cargo run
   Compiling chapter10 v0.1.0 (file:///projects/chapter10)
error[E0106]: missing lifetime specifier
 --> src/main.rs:9:33
  |
9 | fn longest(x: &str, y: &str) -> &str {
  |               ----     ----     ^ expected named lifetime parameter
  |
  = help: **this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `x` or `y`**
help: **consider introducing a named lifetime parameter**
  |
9 | fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
  |           ++++     ++          ++          ++

For more information about this error, try `rustc --explain E0106`.
error: could not compile `chapter10` due to previous error
```

The error we got talks about lifetimes, the return type needs a generic lifetime parameter on it because Rust can’t tell whether the reference being returned refers to `x` or `y`.  And actually we don’t know either because the `if` block in the body of this function returns a reference to `x` and the `else` block returns a reference to `y`!

To fix this error, we’ll add generic lifetime parameters that define the relationship between the references so the borrow checker can perform its analysis.

### Lifetime Annotation Syntax

- Lifetime annotations don’t change how long any of the references live.
- Lifetime annotations describe the relationships of the lifetimes of multiple references to each other without affecting the lifetimes.
- Lifetime annotations have a slightly unusual syntax: the names of lifetime parameters must start with an apostrophe (`'`) and are usually all lowercase and very short, like generic types.

Here are some examples:

```rust
&i32        // a reference
&'a i32     // a reference with an explicit lifetime
&'a mut i32 // a mutable reference with an explicit lifetime
```

### Lifetime annotations in function signatures

- When annotating lifetimes in functions, the annotations go in the function signature, not in the function body.

Now let’s implement lifetime annotations in our previously written longest function.

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

This code should compile and produce the result we want when we use it with the `main` function.

The function tells Rust that for some lifetime `'a`, the function takes two parameters, both of which are string slices that live at least as long as lifetime `'a` and also the string slice returned from the function will live at least as long as lifetime `'a`.

Because we’ve annotated the returned reference with the same lifetime parameter `'a`, the returned reference will also be valid for the length of the smaller of the lifetimes of `x` and `y`.

Consider the following example:

```rust
fn main() {
    let string1 = String::from("long string is long");
    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

>> The longest string is long string is long
```

- `string1` is valid until the end of the outer scope
- `string2` is valid until the end of the inner scope
- `result` references something that is valid until the end of the inner scope.

Now let’s make another example where we move the variable result outside the inner scope:

```rust
fn main() {
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        **result = longest(string1.as_str(), string2.as_str());**
    }
		// string2 doesn't live outside the scope where the println! function gets called
		// string2 would need to be valid until the end of the outer scope
		// string1 is longer than string2 and therefore result will contain a reference to string1
    **println!("The longest string is {}", result);**
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

>> $ cargo run
   Compiling chapter10 v0.1.0 (file:///projects/chapter10)
error[E0597]: `string2` does not live long enough
 --> src/main.rs:6:44
  |
6 |         result = longest(string1.as_str(), string2.as_str());
  |                                            ^^^^^^^^^^^^^^^^ borrowed value does not live long enough
7 |     }
  |     - `string2` dropped here while still borrowed
8 |     println!("The longest string is {}", result);
  |                                          ------ borrow later used here

For more information about this error, try `rustc --explain E0597`.
error: could not compile `chapter10` due to previous error
```

### Thinking in terms of Lifetimes

The way in which you need to specify lifetime parameters depends on what your function is doing. For example, if we changed the implementation of the `longest`
 function to always return the first parameter rather than the longest string slice, we wouldn’t need to specify a lifetime on the `y`
 parameter. The following code will compile:

```rust
/* we’ve specified a lifetime parameter 'a for the parameter x and the return type, but not for the parameter y,
because the lifetime of y does not have any relationship with the lifetime of x or the return value. */
fn longest<'a>(x: &'a str, y: &str) -> &'a str {
    x
}
```

Consider this attempted implementation of the `longest` function that won’t compile:

```rust
fn longest<'a>(x: &str, y: &str) -> &'a str {
    let result = String::from("really long string");
    result.as_str()
}

>> $ cargo run
   Compiling chapter10 v0.1.0 (file:///projects/chapter10)
error[E0515]: cannot return reference to local variable `result`
  --> src/main.rs:11:5
   |
11 |     result.as_str()
   |     ^^^^^^^^^^^^^^^ returns a reference to data owned by the current function

For more information about this error, try `rustc --explain E0515`.
error: could not compile `chapter10` due to previous error
```

Lifetime syntax is about connecting the lifetimes of various parameters and return values of functions.

### Lifetime Annotations in Struct Definitions

- So far, we’ve only defined structs to hold owned types. It’s possible for structs to hold references, but in that case we would need to add a lifetime annotation on every reference in the struct’s definition.

```rust
// This struct has one field, part, that holds a string slice, which is a reference.
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
		println!("{}", first_sentence);
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}

>> Call me Ishmael
```

The `main` function here creates an instance of the `ImportantExcerpt` struct that holds a reference to the first sentence of the `String` owned by the variable `novel`. The data in `novel`
 exists before the `ImportantExcerpt` instance is created. In addition, `novel` doesn’t go out of scope until after the `ImportantExcerpt` goes out of scope, so the reference in the `ImportantExcerpt` instance is valid.

### Lifetime Elision

Let’s take a function that compiles without lifetime annotations:

```rust
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
```

In early versions (pre-1.0) of Rust, this code wouldn’t have compiled because every reference needed an explicit lifetime. At that time we would’ve had to write it like this:

```rust
fn first_word<'a>(s: &'a str) -> &'a str {
```

The Rust team found that Rustaceans spent a lot of time writting lifetime annotations over and over, so they decided to get rid of it. This is relevant to Rust history because in the future, even fewer lifetime annotations might be required.

### Lifetime Annotations in Method Definitions

Lifetime names for struct fields always need to be declared after the `impl` keyword and then used after the struct’s name, because those lifetimes are part of the struct’s type.

```rust
impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
}
```

### The Static Lifetime

- One special lifetime we need to discuss is `'static`, which means that this reference *can* live for the entire duration of the program.

Here is an example:

```rust
let s: &'static str = "I have a static lifetime.";
// The text of this string is stored directly in the program’s binary, which is always available.
```

### Generic Types Parameters, Trait Bounds and Lifetimes Together

Let’s look at the 3 all in one function:

```rust
fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest_with_an_announcement(
        string1.as_str(),
        string2,
        "Today is someone's birthday!",
    );
    println!("The longest string is {}", result);
}

use std::fmt::Display;
// return the longer of two strings
// declaration of lifetime parameter
fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
// Display trait is specified by the where clause
where
    T: Display,
{
		// ann is of generic type
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

### Summary

- With all this knowledge we can write code without repetition that works in many different situations.
- Traits and trait bounds ensure that even though the types are generic, they’ll have the behavior the code needs. You learned how to use lifetime annotations to ensure that this flexible code won’t have any dangling references.