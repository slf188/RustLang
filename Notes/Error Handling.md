# Error Handling

- Errors are a fact of life in software, so Rust has a number of features for handling situations in which something goes wrong.
- Rust groups error into two major categories:
    - **Recoverable:** A file not found error, we want to report the problem and retry the operation.
    - **Unrecoverable:** Trying to access a location beyond the end of an array, and stop the program immediately.
- Rust doesn’t have exceptions, instead it has the type `Result<T, E>` for errors and the `panic!` macro that stops the execution when the program encounters an unrecoverable error.

## Unrecoverable Errors with `panic!`

- When the `panic!` macro executes, your program will print a failure message, unwind and clean up the stack, and then quit.

**Unwinding the stack or aborting in response to a Panic**

By default when a panic occurs, the program starts *unwinding*, which means Rust walks back up the stack and cleans up the data from each function it encounters. This involves a lot of work in the back so, Rust allows us to choose the alternative of immediately *aborting*, which ends the program without cleaning up.

To switch from unwinding to aborting immediately upon a panic add the following piece in the `[profile]` sections in your Cargo.toml file:

```rust
[profile.release]
panic = 'abort'
```

Let’s try calling panic in a simple program:

```rust
fn main() {
    panic!("crash and burn");
}

» $ cargo run
   Compiling panic v0.1.0 (file:///projects/panic)
    Finished dev [unoptimized + debuginfo] target(s) in 0.25s
     Running `target/debug/panic`
thread 'main' panicked at 'crash and burn', src/main.rs:2:5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

### Using a `panic!` backtrace

Let’s see another example where the `panic!` gets called because of a bug in our code instead of being called from our code calling the macro directly:

```rust
fn main() {
    let v = vec![1, 2, 3];
		// access the 100th element of the v vector
    v[99];
		// Rust will stop execution and refuse to continue...
}

» $ cargo run
   Compiling panic v0.1.0 (file:///projects/panic)
    Finished dev [unoptimized + debuginfo] target(s) in 0.27s
     Running `target/debug/panic`
thread 'main' panicked at 'index out of bounds: the len is 3 but the index is 99', src/main.rs:4:5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

In this situation Rust will panic because our vector only has 3 elements. In C, when we attempt to read beyond the end of a data structure is undifined behaviour, you might get whatever is at the location of memory that would correspond to that element in the data structure:

```cpp
int main() {
  int A[] = {1, 2, 3, 4};
	int num = A[99];
	cout << num << endl;
}

» 1775616808
```

This behaviour is called *buffer overread* and it can lead to security vulnerabilities if a user is able to access data that is not within a data structure.

Notice that in the error of our Rust code we get something called a backtrace. A *backtrace*
 is a list of all the functions that have been called to get to this point. We can get a backtrace by setting `RUST_BACKTRACE` to any value except 0:

```cpp
$ RUST_BACKTRACE=1 cargo run
thread 'main' panicked at 'index out of bounds: the len is 3 but the index is 99', src/main.rs:4:5
stack backtrace:
   0: rust_begin_unwind
             at /rustc/7eac88abb2e57e752f3302f02be5f3ce3d7adfb4/library/std/src/panicking.rs:483
   1: core::panicking::panic_fmt
             at /rustc/7eac88abb2e57e752f3302f02be5f3ce3d7adfb4/library/core/src/panicking.rs:85
   2: core::panicking::panic_bounds_check
             at /rustc/7eac88abb2e57e752f3302f02be5f3ce3d7adfb4/library/core/src/panicking.rs:62
   3: <usize as core::slice::index::SliceIndex<[T]>>::index
             at /rustc/7eac88abb2e57e752f3302f02be5f3ce3d7adfb4/library/core/src/slice/index.rs:255
   4: core::slice::index::<impl core::ops::index::Index<I> for [T]>::index
             at /rustc/7eac88abb2e57e752f3302f02be5f3ce3d7adfb4/library/core/src/slice/index.rs:15
   5: <alloc::vec::Vec<T> as core::ops::index::Index<I>>::index
             at /rustc/7eac88abb2e57e752f3302f02be5f3ce3d7adfb4/library/alloc/src/vec.rs:1982
   6: panic::main // here the backtrace points to the line where the error occurs
             at ./src/main.rs:4
   7: core::ops::function::FnOnce::call_once
             at /rustc/7eac88abb2e57e752f3302f02be5f3ce3d7adfb4/library/core/src/ops/function.rs:227
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```

## Unrecoverable Errors with `Result`

- Most errors aren’t serious enough to require the program to stop entirely. Sometimes, we just need to make a few adjustments instead of terminating the entire process.

Recall, the `Result` enum is defined as having two variants, `Ok` and `Err`:

```rust
enum Result<T, E> {
		// T represents the type of the value that will be returned in a success case within the Ok variant
    Ok(T),
		// E represents the type of the error that will be returned in a failure case within the Err variant
    Err(E),
}
```

Let’s call a function that returns a `Result` value because the function could fail, by opnening a non existing file, let’s make the variable store data of type `ur32` and see how the compiler handles the request:

```rust
use std::fs::File;

fn main() {
    let f: u32 = File::open("hello.txt");
}

>> $ cargo run
   Compiling error-handling v0.1.0 (file:///projects/error-handling)
error[E0308]: mismatched types
 --> src/main.rs:4:18
  |
4 |     let f: u32 = File::open("hello.txt");
  |            ---   ^^^^^^^^^^^^^^^^^^^^^^^ expected `u32`, found enum `Result`
  |            |
  |            expected due to this
  |
  = note: expected type `u32`
             found enum `Result<File, std::io::Error>`

For more information about this error, try `rustc --explain E0308`.
error: could not compile `error-handling` due to previous error
```

In the case where `File::open` succeeds, the value of `f` will be an instance of `Ok` that contains a file handle. In the case where it fails, the value in `f` wil be an instance of `Err` that contains more information about the kind of error that happened. We need to add extra code in our previous program, to take different actions depeneding on the value of `File::Open`:

```rust
use std::fs::File;

fn main() {
    let f = File::open("hello.txt");

    let f = match f {
				// this will return the inner file value
        Ok(file) => file,
				// call the panic macro if there is no hello.txt file in our current directory
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
}
```

### Matching on different Errors

The previous program, can be very limited when it comes to the type of error that may appear and how the program reacts to those errors, but, we want extra functionality, we want our program to take different actions for different failure reasons: 

- If `File::open` failed because the file doesn’t exist, we want to create the file and return the handle to the new file.
- If we didn’t have permission to open the file—we still want the code to `panic!` in the same way

To add this extra functionality, we add the following:

```rust
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
				// here we use the io::Error struct, which has a method kind that we can call to get a io::ErrorKind value
        Err(error) => match error.kind() {
						// this error kind indicates that the file we're trying to open doesn't exist, if it run we create a file with File::create
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
								// File::create can fail so we need to handle it as well
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        },
    };
}
```

**Alternatives to Using `match` with `Result<T, E>`**

In chapter 13, we will review functional language features, such as iterators and closures. These extra Rust features can help us write code that is more consice whenever we are writting methods. For example here is another way we can write the previous program using closures:

```rust
use std::fs::File;
use std::io::ErrorKind;
// this program does not contain any match expressions and is cleaner to read
fn main() {
    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
}
```

### Shorcuts for Panic on Error: unwrap and expect

- Using `match` works well enough, but it can be a bit verbose and doesn’t always communicate intent well.
- The `Result<T, E>`type has many helper methods defined on it to do various, more specific tasks. The `unwrap` method is a shortcut for implementing `match` expressions. So if the `Result` value is the `Ok` variant, unwrap will return the value inside `Ok`. If the `Result` is the `Err` variant, unwrap will call the `panic!` macro for us

Here is an example of how we might use the unwrap shortcut:

```rust
use std::fs::File;

fn main() {
    let f = File::open("hello.txt").unwrap();
}

>> thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Error {
repr: Os { code: 2, message: "No such file or directory" } }',
src/libcore/result.rs:906:4
```

- Similarly, the `expect` method let’s us choose the `panic!` error message. Using `expect` instead of `unwrap` and providing a good error message can make the debugging process less painful.

Here is an example of how to use the expect shortcut:

```rust
use std::fs::File;

fn main() {
    let f = File::open("hello.txt").expect("Failed to open hello.txt");
}

>> thread 'main' panicked at 'Failed to open hello.txt: Error { repr: Os { code:
2, message: "No such file or directory" } }', src/libcore/result.rs:906:4
```

### Propagating Errors

- When a function calls something that might fail, instead of handling the error within the function itself, you can return the error to the calling of the code so that it decides what to do.

For example, the following program shows a function that reads a username from a file. If the file doesn’t exist or can’t be read, this function will return those errors to the code that called the function.

```rust
use std::fs::File;
use std::io::{self, Read};
// this means the function is returning a value of the type Result<T, E>. T is of type String and E of type io::Error
// if this function succeeds, it will receive an Ok value that holds a String
// if this function fails, it will receive an Err value of type io::Error
fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
				// here instead of using the panic! macro, we return the error message
        Err(e) => return Err(e),
    };

    let mut s = String::new();
		// here we read the contents of the file f into s
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
				// we don't need to explicitly say return, because is the last expression of the function
        Err(e) => Err(e),
    }
}
```

This pattern of propagating errors is so common in Rust that Rust provides the question mark operator `?` to make this easier.

### A shorcut for propagating errors using the `?` operator

Let’s reduce the amount of code of the previous program using the ? operator:

```rust
use std::fs::File;
use std::io;
use std::io::Read;

fn read_username_from_file() -> Result<String, io::Error> {
		// the ? at the end of this line will return the value inside an Ok to the variable f
    let mut f = File::open("hello.txt")?; // if an error occurs the ? operator will return the whole function plus any Err
    let mut s = String::new();
		// the ? operator does the same to read_to_string
    f.read_to_string(&mut s)?;
    Ok(s)
}
```

The `?` operator eliminates a lot of boilerplate and makes this function’s implementation simpler. We could even shorten this code further by chaining method calls immediately after the `?` as shown:

```rust
use std::fs::File;
use std::io;
use std::io::Read;

fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();
		// instead of creating a variable f, we've chained the call read_to_string directly onto the result of File::open()
    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}
```

We can make this even shorter by writting:

```rust
use std::fs;
use std::io;

fn read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}
```

Reading a file into a string is a fairly common operation, so the standard library provides the convenient `fs::read_to_string` function that opens the file, creates a new `String`, reads the contents of the file, puts the contents into that `String`, and returns it.

### Where the `?` operator can be used

- The `?` operator can only be used in functions whose return type is compatible with the value the `?` is used on.

Let’s look at a misuse of the ? operator:

```rust
use std::fs::File;

fn main() {
		// this program fails because the ? operator returns a Result value, but this main function has the return type of (), not Result
    let f = File::open("hello.txt")?;
}

>> $ cargo run
   Compiling error-handling v0.1.0 (file:///projects/error-handling)
error[E0277]: the `?` operator can only be used in a function that returns `Result` or `Option` (or another type that implements `FromResidual`)
 --> src/main.rs:4:36
  |
3 | / fn main() {
4 | |     let f = File::open("hello.txt")?;
  | |                                    ^ cannot use the `?` operator in a function that returns `()`
5 | | }
  | |_- **this function should return `Result` or `Option` to accept `?`**
  |
  = help: the trait `FromResidual<Result<Infallible, std::io::Error>>` is not implemented for `()`

For more information about this error, try `rustc --explain E0277`.
error: could not compile `error-handling` due to previous error
```

To fix this error, we have either two choices:

- One choice is to change the return type of your function to be compatible with the value you’re using the `?` operator on as long as you have no restrictions preventing that.
- The other technique is to use a `match` or one of the `Result<T, E>` methods to handle the `Result<T, E>` in whatever way is appropriate.

Here is a function that finds the last character of the first line in the given text:

```rust
// This function returns Option<char> because it’s possible that there is a character there, but it’s also possible that there isn’t.
fn last_char_of_first_line(text: &str) -> Option<char> {
		// This code takes the text string slice argument and calls the lines method on it, which returns an iterator over the lines in the string.
    text.lines().next()?.chars().last()
}
```

So far, all the `main` functions we’ve used return `()`. The `main` function is special because it’s the entry and exit point of executable programs. Luckily, `main` can also return a `Result<(), E>`:

```rust
use std::error::Error;
use std::fs::File;
// Box<dyn Error> means "any kind of error"
fn main() -> Result<(), Box<dyn Error>> {
    let f = File::open("hello.txt")?;

    Ok(())
}
```

We can fix the previous function where we tried to use the ? operator by changing the return type of the main function:

## To `panic!` or not to `panic!`

- So the question now is, when should we call `panic!` and when you should return `Result`? Let’s clear things up:
- When code panics, there’s no way to recover.
- When we return a `Result` value, we have more options when it comes to handling the error.

### Examples, Prototype Code and Tests

- When we are writting an example to illustrate some concept, including error-handling can make the whole example less clear.
- The `unwrap` and `expect` methods are very handy when prototyping, before we are ready to decide how to handle errors.

### Cases in which you have more information than the compiler

It would also be appropriate to call `unwrap` when you have some other logic that ensures the `Result` will have an `Ok` value, but the logic isn’t something the compiler understands.

```cpp
use std::net::IpAddr;
// create a ipaddr instance by parsing a hardcoded string
let home: IpAddr = "127.0.0.1".parse().unwrap();
```

### Guidelines for Error Handling

- It’s advisable to have your code panic when it’s possible that your code could end up in a bad state.
- If someone calls your code and passes in values that don’t make sense, the best choice might be to call `panic!` and alert the person using your library to the bug in their code so they can fix it during development.
- When failure is expected, it’s more appropriate to return a `Result` than to make a `panic!` call.
- Functions often have *contracts*: their behavior is only guaranteed if the inputs meet particular requirements. Panicking when the contract is violated makes sense because a contract violation always indicates a caller-side bug and it’s not a kind of error you want the calling code to have to explicitly handle.

### Summary

- Rust’s error handling features are designed to help you write more robust code. By robust we mean our code is able to tolerate perturbations that might affect our code’s core functionality.
- The `panic!` macro signals that your program is in a state it can’t handle and lets you tell the process to stop instead of trying to proceed with invalid or incorrect values.
- You can use `Result` to tell code that calls your code that it needs to handle potential success or failure as well.
- Using `panic!` and `Result` in the appropriate situations will make your code more reliable in the face of inevitable problems.