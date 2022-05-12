# An I/O Project: Building a command line program

- Now it’s time to build something, we will build a command line tool that interacts with file and command line input/output to practice some of the Rust concepts you now have under your belt.
- In this case we will make our own version of the `grep` command (**g**lobally search a **r**egular **e**xpression and **p**rint)
- The grep command works the following way, it takes two arguments a filename as an argument and a string as an argument, it reads the file, finds the line in the file that contain the string argument, and prints those lines.

| Command | Description |
| --- | --- |
| grep [search_pattern][file] | Search for all lines that contain the pattern, e.g. grep "Tom" file.txt |
- One Rust community member, Andrew Gallant, has already created a fully featured, very fast version of `grep`, called `ripgrep`.

## Accepting Command Line Arguments

- Create a new project called minigrep.
- The first task is to make the project accept two command line arguments: the filename and a string to search for.

```rust
$ cargo run searchstring example-filename.txt
```

### Reading the argument values

- To read values from command line arguments, we’ll need a function from the Rust standard library called `std::env::args`. This function returns an iterator of the command line arguments.
- Iterators produce a series of values, we can call the `collect` method on an iterator to turn it into a collection, like a vector, containing all the elements.

Here is the way to use it:

```rust
// It’s conventional to bring the parent module into scope rather than the function
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
}
```

<aside>
⚠️ Note that `std::env::args` will panic if any argument contains invalid Unicode. If your program needs to accept arguments containing invalid Unicode, use `std::env::args_os` instead.

</aside>

- We can use the `collect` function to create many kinds of collections, but we explicitly annotate the type of `args`  to specify that we want a vector of strings.

### Saving the argument values in variables

- By printing the values of the vector illustrates that the program is able to access those values specified as command line arguments.

In order to save those arguments, we can do so by using:

```rust
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let filename = &args[2];

    println!("Searching for {}", query);
    println!("In file {}", filename);
}
```

Let us run this program using two arguments:

```rust
$ cargo run test sample.txt
   Compiling minigrep v0.1.0 (file:///projects/minigrep)
    Finished dev [unoptimized + debuginfo] target(s) in 0.0s
     Running `target/debug/minigrep test sample.txt`
Searching for test
In file sample.txt
```

## Reading a file

- Now let’s add some functionality to read the file that is specified in the cli arguments, so let’s add some small amount of text onto the file with some repeated words.
- Create a file poem.txt and enter the following piece of text:

```markdown
I'm nobody! Who are you?
Are you nobody, too?
Then there's a pair of us - don't tell!
They'd banish us, you know.

How dreary to be somebody!
How public, like a frog
To tell your name the livelong day
To an admiring bog!
```

Now we add code to read the file:

```rust
use std::env;
// fs is the function we need to handle files
use std::fs;

fn main() {
    // --snip--
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let filename = &args[2];

    println!("Searching for {}", query);
    println!("In file {}", filename);

		// read_to_string takes the filename, opens the file and returns Result<String>
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    println!("With text:\n{}", contents);
}

» cargo run somebody poem.txt
Compiling minigrep v0.1.0 (/Users/felipe/Documents/Rust/minigrep)
    Finished dev [unoptimized + debuginfo] target(s) in 0.51s
     Running `target/debug/minigrep somebody poem.txt`
Searching for somebody
In file poem.txt
With text:
I'm nobody! Who are you?
Are you nobody, too?
Then there's a pair of us - don't tell!
They'd banish us, you know.

How dreary to be somebody!
How public, like a frog
To tell your name the livelong day
To an admiring bog!
```

This program works fine for now but, these flaws aren’t a big problem, but as the program grows, it will be harder to fix them cleanly. It’s good practice to begin refactoring early on when developing a program, because it’s much easier to refactor smaller amounts of code.

## Refactoring to improve Modularity and Error Handling

- Currently our main function handles two tasks, it parses arguments and reads files. This works fine because the main function contains just a few lines of code but we better separate functionality so each function is responsible for one task at a time.
- Another problem is we’ve used `expect` to print an error message when reading the file fails. But the real problem is reading a file can fail in a number of ways, the file could be missing, or we might not have permission to open it.

### Separation of concerns for binary projects

- Allocating responsibility for multiple tasks to the `main` function is common to many binary projects.
- As a result the Rust community developed a process to split separate concerns of a binary program when `main` starts getting large.
- The process contains the following steps:
    - Split your program into a *main.rs* and a *lib.rs* and move your program’s logic to *lib.rs*.
    - As long as your command line parsing logic is small, it can remain in *main.rs*.
    - When the command line parsing logic starts getting complicated, extract it from *main.rs* and move it to *lib.rs*.
- The only responsabilities the main function should have are:
    - Calling the command line parsing logic with the argument values
    - Setting up any other configuration
    - Calling a `run` function in *lib.rs*
    - Handling the error if `run` returns an error
- In short *main.rs* handles running the program, and *lib.rs* handles all the logic of the task at hand.

### Extracting the argument parser

We’ll extract the functionality for parsing arguments into a function that `main` will call:

```rust
fn main() {
    let args: Vec<String> = env::args().collect();
		/* now instead of assigning the argument of index 1 to variable 
		query and index 2 to variable filename within the main function we use the parse_config function */
    let (query, filename) = parse_config(&args);
}

fn parse_config(args: &[String]) -> (&str, &str) {
    let query = &args[1];
    let filename = &args[2];

    (query, filename)
}
```

### Grouping configuration values

- We can improve the `parse_config` function even further. At the moment we are returning a tuple, but then we immediately break that tuple into individual parts again.
- This is something we may want to fix because we could instead use a struct and give each of the struct fields a meaningful name, this will help other coders understand our code

```rust
fn main() {
    let args: Vec<String> = env::args().collect();

    let config = parse_config(&args);

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    let contents = fs::read_to_string(config.filename)
        .expect("Something went wrong reading the file");
}

struct Config {
    query: String,
    filename: String,
}
// now our function returns a Config struct
fn parse_config(args: &[String]) -> Config {
    let query = args[1].clone();
    let filename = args[2].clone();

    Config { query, filename }
}
```

### Creating a Constructor for Config

- Now that the purpose of the `parse_config` function is to create a `Config` instance, we can change `parse_config` from a plain function to a function named `new` that is associated with the `Config` struct.

```rust
fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);
}

impl Config {
    fn new(args: &[String]) -> Config {
        let query = args[1].clone();
        let filename = args[2].clone();

        Config { query, filename }
    }
}
```

### Fixing the error handling

- Recall that attempting to access the values in the `args` vector at index 1 or index 2 will cause the program to panic if the vector contains fewer than three items.

If we run the program without any arguments it will look like this:

```rust
$ cargo run
   Compiling minigrep v0.1.0 (file:///projects/minigrep)
    Finished dev [unoptimized + debuginfo] target(s) in 0.0s
     Running `target/debug/minigrep`
thread 'main' panicked at '**index out of bounds: the len is 1 but the index is 1**', src/main.rs:27:21
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

The error marked in bold letters is intended for programmers. It won’t help our users understand what happened and how they should react.

### Improving the error message

Let’s add extra functionality to the new function and try to verify if the slice is long enough before accessing index 1 or 2.

```rust
fn new(args: &[String]) -> Config {
        if args.len() < 3 {
            panic!("not enough arguments");
        }
}

» $ cargo run
   Compiling minigrep v0.1.0 (file:///projects/minigrep)
    Finished dev [unoptimized + debuginfo] target(s) in 0.0s
     Running `target/debug/minigrep`
thread 'main' panicked at '**not enough arguments**', src/main.rs:26:13
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

### Returning a Result from new instead of calling panic!

- We can instead return a `Result` value that will contain a `Config` instance in the successful case and will describe the problem in the error case.

```rust
impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}
```

No instead of calling `panic!` when the user doesn’t pass enough arguments, we now return an `Err` value, and we’ve wrapped the `Config` return value in an `Ok`.

### Calling Config::new and handling Errors

To handle the err case and the Result returned from the Config::new we must do the following changes in main:

```rust
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
// Using unwrap_or_else allows us to define some custom, non-panic! error handling.
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
}

» $ cargo run
   Compiling minigrep v0.1.0 (file:///projects/minigrep)
    Finished dev [unoptimized + debuginfo] target(s) in 0.48s
     Running `target/debug/minigrep`
Problem parsing arguments: not enough arguments
```

### Extracting Logic from main

- Now we can start by extracting a function named `run` that will hold all the logic currently in the `main` function that isn’t involved with setting up configuration or handling errors.

```rust
fn main() {
    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    run(config);
}

fn run(config: Config) {
    let contents = fs::read_to_string(config.filename)
        .expect("Something went wrong reading the file");

    println!("With text:\n{}", contents);
}
```

The `run` function now contains all the remaining logic from `main`, starting from reading the file.

### Returning Errors from the run function

- Let us improve the error handling for this new run function, instead of allowing the program to panic by calling `expect`, the `run` function will return a `Result<T, E>` when something goes wrong.

```rust
use std::error::Error;
fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    println!("With text:\n{}", contents);

    Ok(())
}
```

Changes:

1. `Box<dyn Error>` means the function will return a type that implements the `Error` trait, but we don’t have to specify what particular type the return value will be.
2. Rather than `panic!` on an error, `?` will return the error value from the current function for the caller to handle.
3. The `run` function now returns an `Ok` value in the success case.

### Handling Errors returned from run in main

Let’s use a new technique to handle new errors:

```rust
fn main() {
    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    if let Err(e) = run(config) {
        println!("Application error: {}", e);

        process::exit(1);
    }
}
```

We use `if let` rather than `unwrap_or_else` to check whether `run` returns an `Err` value and call `process::exit(1)` if it does.

### Splitting code into a library crate

- Our project is starting to work, now it’s time to split the src/main.rs file and put some code into src/lib.rs so we can test it.
- The pieces of code we can run into src/lib.rs are:
    - The run function
    - The use statements
    - The definition of Config
    - The Config::new function definition

Filename: src/lib.rs

```rust
use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        // --snip--
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // --snip--
}
```

We’ve made extensive use of the `pub` keyword. We now have a library crate that has a public API that we can test! Which means we can use some of the code that is within src/lib.rs into src/main.rs

Filename: src/main.rs

```rust
use std::env;
use std::process;
// we add use minigrep::Config to bring the Config library into the main.rs file
use minigrep::Config;

fn main() {
    // --snip--
    if let Err(e) = minigrep::run(config) {
        // --snip--
    }
}
```

## Developing the library’s functionality with test-driven development

- Now that we have extracted the logic into src/lib.rs and left the argument collecting and error handling in src/main.rs it is much easier to test functionality of our code.
- In this part we will focus on building the functionality for searching. We will use test-driven development, this technique follows these steps:
    1. Write a test that fails and run it to make sure it fails for the reason you expect.
    2. Write or modify just enough code to make the new test pass.
    3. Refactor the code you just added or changed and make sure the tests continue to pass.
    4. Repeat from step 1!
- This process is one of the many ways to write software, but TDD can help a lot in many ways.
- We will add the searching functionality in a `search` function.

### Writing a failing test

- We don’t need the `println!` statements inside the src/lib.rs and src/main.rs files so we can remove them.
- We will add the `tests` module inside src/lib.rs with a test function. The test funcion specifies the behaviour we want the `search` function to have: it will take a query and the text for search in the query, and it will finally return onlly the lines from the text that contain the query.

Filename: src/lib.rs

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
				// the test searches for duct
        let query = "duct";
				// the text in which we are searching is the contents variable
        // the backslash after the opening quotes tells Rust not to put a newline char at the beginning of our string
				let contents = "\ 
Rust:
safe, fast, pro**duct**ive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}
```

Because the `search` function doesn’t exist yet, we must add just enough code into the function to test it out, let’s make the function return an empty vector, the function definition should be before the tests module:

Filename: src/lib.rs

```rust
// we need a lifetime to specify which arguments lifetime is connected to the lifetime of the return value
/* in other words, we tell rust that the data 
returned by the search function will live as long as the data is passed into the search function in the contents argument */
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    vec![]
}
```

- Because `contents` is the argument that contains all of our text and we want to return the parts of that text that match, we know `contents` is the argument that should be connected to the return value using the lifetime syntax.
- Other programming languages don’t require you to connect arguments to return values in the signature. This may seem kind of strange but it will become normal over time.

```rust
$ cargo test
   Compiling minigrep v0.1.0 (file:///projects/minigrep)
    Finished test [unoptimized + debuginfo] target(s) in 0.97s
     Running unittests (target/debug/deps/minigrep-9cd200e5fac0fc94)

running 1 test
test tests::one_result ... FAILED

failures:

---- tests::one_result stdout ----
thread 'main' panicked at 'assertion failed: `(left == right)`
  left: `["safe, fast, productive."]`,
 right: `[]`', src/lib.rs:44:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

failures:
    tests::one_result

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

error: test failed, to rerun pass '--lib'
```

Great the test has failed, let’s get the test to pass!

### Writing code to pass the test

- Currently our test fails, because our function always returns an empty vector. To fix that and implement `search` the program needs to implement the following steps:
    - Iterate through each line of the contents var.
    - Check whether the line contains our query.
    - If the condition is true, add it to the list of values we want to return.
    - If the condition is false, do nothing.
    - Return the list of results, that match.

### Iterating through lines with the `lines` method

- Rust has a helpful method to handle line-by-line ieration of strings, named `lines` that works in the following way:

Filename: src/lib.rs

```rust
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    for line in contents.lines() {
        // do something with line
    }
}
```

The `lines` method returns an iterator.

### Searching each line for the query

- Now let’s check whether the current line contains our query string. Fortunately, strings have a helpful method called `contains` that does this for us:

Filename: src/lib.rs

```rust
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    for line in contents.lines() {
        if line.contains(query) {
            // do something with line
        }
    }
}
```

### Storing matching lines

- Now let us store the lines that contain our query string. For this, we can create a mutable vector before the for loop and call the `push` method to store a `line` in the vector. After the `for` loop ends we return the vector.

Filename: src/lib.rs

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

Now let’s try to run a the test for our function now:

```rust
$ cargo test
   Compiling minigrep v0.1.0 (file:///projects/minigrep)
    Finished test [unoptimized + debuginfo] target(s) in 1.22s
     Running unittests (target/debug/deps/minigrep-9cd200e5fac0fc94)

running 1 test
test tests::one_result ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests (target/debug/deps/minigrep-9cd200e5fac0fc94)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests minigrep

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

Our test passed so we know it works fine!

### Using the search function in the run function

- Now that `search` is working fine and it has been tested, we need to call `search` from our `run` function.

Filename: src/lib.rs

```rust
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    for line in search(&config.query, &contents) {
        println!("{}", line);
    }

    Ok(())
}
```

Now the entire program should work, let’s try it out!

```rust
$ cargo run frog poem.txt
   Compiling minigrep v0.1.0 (file:///projects/minigrep)
    Finished dev [unoptimized + debuginfo] target(s) in 0.38s
     Running `target/debug/minigrep frog poem.txt`
How public, like a frog
```

Cool, let’s try another example:

```rust
$ cargo run body poem.txt
   Compiling minigrep v0.1.0 (file:///projects/minigrep)
    Finished dev [unoptimized + debuginfo] target(s) in 0.0s
     Running `target/debug/minigrep body poem.txt`
I'm no**body**! Who are you?
Are you no**body**, too?
How dreary to be some**body**!
```

And let’s see if we get nothing in case we provide an invalid argument in the cli:

```rust
$ cargo run monomorphization poem.txt
   Compiling minigrep v0.1.0 (file:///projects/minigrep)
    Finished dev [unoptimized + debuginfo] target(s) in 0.0s
     Running `target/debug/minigrep monomorphization poem.txt`
```

## Working with Environment Variables

- Let’s improve our project to allow case-sensitive searching.
- To add this feature we will again use test-driven development to achieve this.

### Writting a failing test for the case sensitive `search` function

- We want to add the new `search_case_sensitive` function that will get triggered when the environment variable is on.
- We must follow TDD, so the first step is to write a failing test.
- First let’s rename our old test from `one_result` to `case_sensitive` to clarify the difference between the two tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT"; // we will create a function that will match the line "Rust:" and "Trust me"
				// the query should work dispite the different casing from the query
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
// In this case the first test should pass but the second test will fail
```

### Implementing the `search_case_sensitive` function

- This function will be almost the same as the `search` function. The only difference is that we will lower case the query and so whatever the case of the input arguments, they’ll be the same as the old `search` function.

```rust
pub fn search_case_insensitive<'a>(
    query: &str,
    contents: &'a str,
) -> Vec<&'a str> {
		// lowe the query string
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}
```

If we run src/lib.rs we will get:

```rust
$ cargo test
   Compiling minigrep v0.1.0 (file:///projects/minigrep)
    Finished test [unoptimized + debuginfo] target(s) in 1.33s
     Running unittests (target/debug/deps/minigrep-9cd200e5fac0fc94)

running 2 tests
test tests::case_insensitive ... ok
test tests::case_sensitive ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests (target/debug/deps/minigrep-9cd200e5fac0fc94)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests minigrep

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

Now let’s try to call the new `search_case_sensitive` function from the `run` function. Let’s add the configuration to the `Config` struct to switch between cases.

```rust
pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}
```

Note it holds a bool, so let’s go to the `run` function to check the `case_sensitive` value and decide whether to call `search` or `search_case_sensitive`.

```rust
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}
```

Finally we need to check for the environment variables. To work with environment variables we need to use the `use std::env;` module in the standard library. Then we need to use the `var` function from the `env` module to check for a variable named `CASE_INSENSITIVE` :

```rust
use std::env;

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

Now let’s try to run our program and see if it works:

```rust
$ cargo run to poem.txt
   Compiling minigrep v0.1.0 (file:///projects/minigrep)
    Finished dev [unoptimized + debuginfo] target(s) in 0.0s
     Running `target/debug/minigrep to poem.txt`
Are you nobody, too?
How dreary to be somebody!
```

## Writting error messages to standard error instead of standard ouput

- At the moment, we’re writing all of our output to the terminal using the `println!` macro.
- In most terminals, there are two kinds of output: *standard output* (`stdout`) for general information and *standard error* (`stderr`) for error messages.
- The `println!` macro is only capable of printing to standard output, so we have to use something else to print to standard error.

### Checking where errors are written

If we write use the `>` symbol, we can redirect the standard output stream to a particular filename:

```rust
$ cargo run > output.txt
```

The `>` syntax tells the shell to write the contents of standard output to *output.txt* instead of the screen. This is what output.txt contains:

```rust
Problem parsing arguments: not enough arguments
```

### Printing errors to standard error

Let’s change how errors are printed.

- The standard library provides the `eprintln!` macro that prints to the standard error stream, so instead of using `println!` let’s start using `eprintln!`.

```rust
fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);

        process::exit(1);
    }
}

» $ cargo run > output.txt
Problem parsing arguments: not enough arguments
```

Now we see the error onscreen and *output.txt* contains nothing, this is the behaviour we expect from command line programs

Again let’s try to run the program with arguments that won’t cause errors:

```rust
$ cargo run to poem.txt > output.txt
```

We won’t see any output to the terminal, and *output.txt* will contain our results:

Filename: output.txt

```rust
Are you nobody, too?
How dreary to be somebody!
```

### Summary

- In this chapter we learned how to perform common I/O operations in Rust.
- We have learned how to use command line arguments, files, environment variables and the `eprintln!` macro for printing errors.
- We can use all this knowledge to write cli apps.
- We have learned about test-driven developement, that makes our code well organized, handle errors nicely and well tested.