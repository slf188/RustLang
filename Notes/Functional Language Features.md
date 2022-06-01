# Functional Language Features: Iterators and Closures

- Rust’s design has taken inspiration from many existing languages and techniques, and one significant influence is *functional programming.*
- Programming in a functional style often includes using functions as values by passing them in arguments, returning them from other functions, assigning them to variables for later execution, and so forth.

In this chapter we will cover many of the functional programming concepts that are applied within Rust:

- Closures: a function like construct we can store in a variable.
- Iterators: a way of processing a series of elements.
- How to use both closures and iterators to improve our previous *minigrep* cli app.
- The performance of the two features.

There are other Rust features which we have already covered, such as pattern matching and enums, that are influenced by the functional style as well.

## Closures: Anonymous functions that can capture their environment

- Rust closures are functions we can save onto a variable or pass as an argument to other functions.
- Unlike functions, closures can capture values from the scope in which they’re defined.

### Creating an abstraction of behaviour with closures

- Let's store a closure so we can use it to be executed later.
- Consider the following situation: we work at a startup that’s making an app to generate custom exercise workout plans. The backend is written in Rust, and the algorithm that generates the workout plan takes into account many factors, such as the app user’s age, body mass index, exercise preferences, recent workouts, and an intensity number they specify. The actual algorithm used isn’t important in this example; what’s important is that this calculation takes a few seconds. We want to call this algorithm only when it is necessary.
- We will call this hypothetical algorithm with the function name of `simulated_expensive_calculation` which will print `calculating slowly...`, wait for two seconds and return whatever number is passed.

```rust
use std::thread;
use std::time::Duration;

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}
```

Next the `main` function represents the code that the app will call when a user asks for a workout plan. The required inputs for the function are:

- An intensity number from the user, to indicate whether they want a low-intensity workout or a high-intensity workout.
- A random number that will generate some variety in the workout plans

The output will be the recommended workout plan:

```rust
fn generate_workout(intensity: u32, random_number: u32) {}

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);
}
```

Here the main function calls a `generate_workout` function with the hard coded input values.

Now that we have the context, let’s see what the algorithm looks like:

```rust
fn generate_workout(intensity: u32, random_number: u32) {
    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            simulated_expensive_calculation(intensity)
        );
        println!(
            "Next, do {} situps!",
            simulated_expensive_calculation(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                simulated_expensive_calculation(intensity)
            );
        }
    }
}
```

This code, but let’s say the data science team decides we need to make some changes to the way we call the `simulated_expensive_calculation` function in the future.

To simplify the update when those changes happen, we want to refactor this code so it calls the `simulated_expensive_calculation`
 function only once.

### Refactoring using functions

- We could restructure the workout program in many ways. But first let’s try to extract the duplicated call to the `simulated_expensive_calculation`
 function into a variable:

```rust
fn generate_workout(intensity: u32, random_number: u32) {
    let expensive_result = simulated_expensive_calculation(intensity);

    if intensity < 25 {
				/* we previously wrote the following piece of code for these two lines:
				println!(
            "Today, do {} pushups!",
            simulated_expensive_calculation(intensity)
        );
        println!(
            "Next, do {} situps!",
            simulated_expensive_calculation(intensity)
        );
				*/
        println!("Today, do {} pushups!", expensive_result);
        println!("Next, do {} situps!", expensive_result);
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", expensive_result);
        }
    }
}
```

This change unifies all the calls to `simulated_expensive_calculation` and solves the problem of the first `if` block unnecessarily calling the function twice. This is a perfect use case for closures!

### Refactoring with closures to store code

- Instead of always calling the `simulated_expensive_calculation` function before the `if` blocks, we can define a closure and store the *closure* in a variable rather than storing the result of the function call i.e. we can define the `simulated_expensive_calculation`within the `generate_workout` function using closures:

```rust
fn generate_workout(intensity: u32, random_number: u32) {
		// the closure definition starts after the = symbol to assign it to the expensive_closure
		// to define a closure we start with a pair of |, inside we specify the parameters to the closure, in this case the param is num
		// if we had more than one parameter then it would be |param1, param2|
		// after the parameters we place curly brackets that hold the body of the closure, these are optional if the closure contains a single expression
		// the value returned from the last line in the closure body num will be the value returned from the closure when it gets called
		// expensive_closure contains the definition of an anonymous function, not the resulting value of calling the anonymous function
		// recall we can use this expensive_closure and call it at a later point, the code we want is stored at expensive_closure
    **let expensive_closure = |num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };**
		// we call the closure like we call a function, we specify the variable name and follow it with parentheses containing the arguments
		// now we’re only executing that code where we need the results.
    if intensity < 25 {
        println!("Today, do {} pushups!", **expensive_closure(intensity)**);
        println!("Next, do {} situps!", **expensive_closure(intensity)**);
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                **expensive_closure(intensity)**
            );
        }
    }
}
```

### Closure type inference and annotation

- Closures don’t require you to annotate the types of the parameters or the return value like `fn` functions do.
- The compiler is reliably able to infer the types of the parameters and the return type.
- Making programmers annotate the types in these small, anonymous functions would be annoying and largely redundant with the information the compiler already has available.
- As with variables, we can add type annotations if we want to increase explicitness and clarity at the cost of being more verbose than is strictly necessary.

If we added type notations into our previosly written closure it will look like this:

```rust
		let expensive_closure = |num: u32| -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };
```

With type notations closures look much more similar to the syntax of functions. Here is a vertical comparison of the syntax for the definition of a function that adds 1 to its parameter and a closure that has the same behaviour:

```rust
// function definition
fn  add_one_v1   (x: u32) -> u32 { x + 1 }
// a fully annotated closure definition
let add_one_v2 = |x: u32| -> u32 { x + 1 };
// no type notations in the closure
let add_one_v3 = |x|             { x + 1 };
// no brackets in the closure
let add_one_v4 = |x|               x + 1  ;
```

These are all valid definitions that will produce the same behavior when they’re called.

For instance here we have a closure that just returns the value it receives as parameter:

```rust
fn main(){
		let example_closure = |x| x;
		// notice this script won't work
    let s = example_closure(String::from("hello")); // we first lock the closure to accept string type parameters
    let n = example_closure(5); // because the closure accepts strings it will requiere us to turn 5 into a string
}

>> $ cargo run
   Compiling closure-example v0.1.0 (file:///projects/closure-example)
error[E0308]: mismatched types
 --> src/main.rs:5:29
  |
5 |     let n = example_closure(5);
  |                             ^- help: **try using a conversion method: `.to_string()`**
  |                             |
  |                             expected struct `String`, found integer

For more information about this error, try `rustc --explain E0308`.
error: could not compile `closure-example` due to previous error
```

The reason for the error is because **the first time we call `example_closure` with the `String` value, the compiler infers the type of `x` and the return type of the closure to be `String`.** Those types are then locked into the closure in `example_closure`, and **we get a type error if we try to use a different type with the same closure.**

### Storing Closures Using Generic Parameters and the fn traits

- Let’s return to our workout app, remember the program was calling the closure more times than it needed to, one option for this would be to save the result of the closure in a variable for reuse and use the variable whenever we need to, however this results in a lot of repeated code.
- Another solution is to create a struct that will hold the closure and the resulting value of calling that closure, the struct will execute only if we need the value, this pattern is called as memoization or lazy evaluation.
- To make such struct that holds a closure, we need to specify the type of the closure because a struct needs to know the types of each of its fields.  All closures within a struct implement at least on of the following traits: `Fn` , `FnMut`or  `FnOnce`. For this example we will only use `Fn`.

To represent the closure to accept parameters and return of type `u32`:

```rust
struct Cacher<T>
where
		// any closure we want to implement shall possess one parameter and return the specified type
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,
}
```

To implement a behaviour within our struct that allows us to call the closure whenever needed we need to use a `Cacher` to ask for the result of a closure:

```rust
impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}
```

The `Cacher::new` function takes a generic parameter `T`, which we’ve defined as having the same trait bound as the `Cacher` struct. Then `Cacher::new` returns a `Cacher` instance that holds the closure specified in the `calculation` field and a `None` value in the `value` field, because we haven’t executed the closure yet.

The value method checks whether we already have a resulting value in `self.value` in a `Some`; if we do, it returns the value within the `Some` without executing the closure again. 

If `self.value` is `None`, the code calls the closure stored in `self.calculation`, saves the result in `self.value` for future use, and returns the value as well.

Here is an example of how we can use our struct within the generate_workout function:

```rust
fn generate_workout(intensity: u32, random_number: u32) {
// Instead of saving the closure in a variable directly, we save a new instance of Cacher that holds the closure.
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(intensity));
        println!("Next, do {} situps!", expensive_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            );
        }
    }
}

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);
}
```

### Limitations of the cacher implementation

- Caching values is a generally useful behavior that we might want to use in other parts of our code with different closures.
- However there are a few problems with the current `Cacher` implementation:

The first problem is that a `Cacher` instance assumes it will always get the same value for the parameter `arg` to the `value` method. It will fail:

```rust
struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    **#[test]
    fn call_with_different_values() {
        let mut c = Cacher::new(|a| a);

        let v1 = c.value(1);
        let v2 = c.value(2);

        assert_eq!(v2, 2);
    }**
}

>> $ cargo test
   Compiling cacher v0.1.0 (file:///projects/cacher)
    Finished test [unoptimized + debuginfo] target(s) in 0.72s
     Running unittests (target/debug/deps/cacher-074d7c200c000afa)

running 1 test
test tests::**call_with_different_values** ... FAILED

failures:

---- tests::call_with_different_values stdout ----
thread 'main' panicked at **'assertion failed: `(left == right)`
  left: `1`,
 right: `2`'**, src/lib.rs:43:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

failures:
    tests::call_with_different_values

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

error: test failed, to rerun pass '--lib'
```

This test creates a new `Cacher` instance with a closure that returns the value passed into it. We call the `value` method on this `Cacher` instance with an `arg` value of 1 and then an `arg` value of 2, and we expect the call to `value` with the `arg` value of 2 to return 2.

### Capturing the environment with closures

- In the workout generator example, we only used closures as inline anonymous functions.
- However, closures have an additional capability that functions don’t have: they can capture their environment and access variables from the scope in which they’re defined.

Here is a quick example:

```rust
fn main() {
    let x = 4;

    let equal_to_x = |z| z == x;

    let y = 4;

    assert!(equal_to_x(y));
}
```

If we try to do the same thing with functions the whole thing will fail:

```rust
fn main() {
    let x = 4;

    fn equal_to_x(z: i32) -> bool {
        z == x
    }

    let y = 4;

    assert!(equal_to_x(y));
}

>> $ cargo run
   Compiling equal-to-x v0.1.0 (file:///projects/equal-to-x)
error[E0434]: can't capture dynamic environment in a fn item
 --> src/main.rs:5:14
  |
5 |         z == x
  |              ^
  |
  = help: **use the `|| { ... }` closure form instead**

For more information about this error, try `rustc --explain E0434`.
error: could not compile `equal-to-x` due to previous error
```

Once again, the compiler reminds us that this will only work if we use a closure form.

The way closures can capture values is in three ways, which directly correlates to the three ways a function can take a parameter: taking ownership, borrowing mutably, and borrowing immutably. These are encoded into three `Fn` traits:

- `FnOnce` consumes the variables it captures from its enclosing scope, known as the closure’s *environment*.
- `FnMut` can change the environment because it mutably borrows values.
- `Fn` borrows values from the environment immutably.

When we create closures, Rust automatically infers which trait to use based on how the closure uses the values in its environment. All closures implement `FnOnce` by default because they can be called at least once. Closures that don’t move the captured variables also implement `FnMut`, and closures that don’t need mutable access to the captured variables also implement `Fn`.

<aside>
💡 `move` converts any variables captured by reference or mutable reference to variables captured by value.

</aside>

```rust
let data = vec![1, 2, 3];
let closure = move || println!("captured {data:?} by value");

// data is no longer available, it is owned by the closure
```

## Processing a series of items with iterators

- The iterator pattern allows you to perform some task on a sequence of items in turn.
- The great thing about using iterators is that it has the capability of iterating over each item within a list and determining when the sequence has finished.
- Once we’ve created an iterator, we can use it in a variety of ways.

Here is a quick example of how iterators work:

```rust
fn main() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("Got: {}", val);
    }
}
```

In languages that don’t support iterators, in order to achieve the same behaviour we would have to do the following: starting a variable at index 0, using that variable to index into the vector to get a value, and incrementing the variable value in a loop until it reached the total number of items in the vector.

Iterators are very handy because they handle all the logic for us, cutting down on repeatitive code that could potentially mess up the codebase.

### The Iterator trait and the `next` method

The definition of the Iterator trait looks like this:

```rust
fn main() {
pub trait Iterator {
    **type Item;**

    fn next(&mut self) -> Option<**Self::Item**>;

    // methods with default implementations elided
		// type Item and Self::Item are defining an associated type with this trait
}
```

The only method that is needed to achieve the iterator functionality is the `next` function:  it returns one item of the iterator at a time wrapped in `Some` and, when iteration is over, returns `None`.

Here is a quick example of how the `next` function works:

```rust
#[cfg(test)]
mod tests {
    **#[test]
    fn iterator_demonstration() {
        let v1 = vec![1, 2, 3];**
				// notice v1_iter needs to be mutable
        **let mut v1_iter = v1.iter();

        assert_eq!(v1_iter.next(), Some(&1));
        assert_eq!(v1_iter.next(), Some(&2));
        assert_eq!(v1_iter.next(), Some(&3));
        assert_eq!(v1_iter.next(), None);
    }**
}
```

## Improving our I/O Project

- With this knowledge of iterators, we can improve our previously written I/O project to make our code cleaner and concise. We shall use the implementation of `Config::new` function and the `search` function.

### Removing a clone using an iterator

- Previously we added code that took a slice of `String` values and created an instance of the `Config` struct by indexing into the slice and cloning the values, allowing the `Config` struct to own those values.

```rust
impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}
```

At the time of writting this implementation we did not worry about the inefficient `clone` calls, it’s time to remove them!

We needed `clone` because we have a slice with `String` elements in the parameter `args`, but the `new` function doesn’t own `args`.

With our knowledge of iterators we can change the `new` function to take ownership of an iterator as its argument instead of borrowing a slice.

### Using the returned iterator directly

Our src/main.rs file which looks like this:

```rust
use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
}
```

Shall be transformed into:

```rust
use std::env;
use std::process;

use minigrep::Config;

fn main() {
		// oijfoeiwjfw
		// The env::args function returns an iterator!
		// now we're passing passing ownership of the iterator returned from env::args to Config::new directly
    **let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });**
		// joijoij
}
```

Next we need to update the definition of `Config::new` in src/lib.rs:

```rust
impl Config {
		// here the type of the iterator returned is std::env::Args instead of &[String]
		// we are taking ownership of the args, we shall modify it to mutable parameter
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        // --snip--
```

### Using the iterator trait methods instead of indexing

Now it’s time to update the next method within the implementation of Config:

```rust
impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        };

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}
```

### Making code clearer with iterator adaptators

We can also take advantage of iterators in the `search` function:

```rust
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}
```

The result would be:

```rust
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}
```

Recall the purpose of the `search` function is to return all lines in contents that contain the query.

## Comparing performance: Loops vs Iterators

- To determine whether to use loops or iterators, we need to learn which implementation is much faster i.e. the comparison between the implementation of the `search` function with an explicit `for` loop.
- In this case we ran the entire contents of The Adventures of Sherlock Holmes by Sir Arthur Conan Doyle into a string and looking for the word *the* in the contents. Here is the result in terms of perfomance:

```rust
// for loop
test bench_search_for  ... bench:  19,620,300 ns/iter (+/- 915,700)
// iterators
test bench_search_iter ... bench:  19,234,900 ns/iter (+/- 657,200)
```

The iterator was slightly faster, iterators are one of Rust’s *zero-cost abstractions*, by which we mean using the abstraction imposes no additional runtime overhead. This is analogous to how Bjarne Stroustrup, the original designer and implementor of C++, defines *zero-overhead* in “Foundations of C++” (2012):

<aside>
💡 In general, C++ implementations obey the zero-overhead principle: What you don’t use, you don’t pay for. And further: What you do use, you couldn’t hand code any better.

</aside>

## Summary

- Closures and iterators are Rust features inspired by functional programming language ideas.
- They contribute to Rust’s capability to clearly express high-level ideas at low-level performance.
- The implementations of closures and iterators are such that runtime performance is not affected.