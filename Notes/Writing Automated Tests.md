# Writing Automated Tests

- In his 1972 essay [“The Humble Programmer”](https://www.cs.utexas.edu/users/EWD/transcriptions/EWD03xx/EWD340.html) Edsger Dijkstra stated that “**Program testing can be a very effective way to show the presence of bugs, but it is hopelessly inadequate for showing their absence.”**. But that doesn’t mean we shouldn’t try to test as much as we can.
- Rust includes support for writing automated software tests within the language.
- As an example, say we write a function called `add_two` that adds 2 to whatever number is passed to it. ****The function accepts integers as parameters and returns an integer. When we compile such function, Rust does all the type checking and borrow checking to ensure we aren’t passing a string type or any other invalid type. But Rust won’t be able to handle a case where say the parameter is -1 or -50. That’s where tests come into play.
- We can write tests that assert, for example, that when we pass `3` to the `add_two` function, the returned value is `5`.

## How to write Tests?

- Tests are Rust functions that verify that the tested code is functioning in the expected manner.
- The bodies of test functions typically perform these three actions:
    1. Set up needed data
    2. Run code we want to test
    3. Assert the result is what we expected
- The main two Rust attributes that will help us with this task is the `test` and `should_panic` attribute.

### The Anatomy of a Test function

- A test in Rust is a function that’s annotated with the `test` attribute.
- To change a function into a test function, add `#[test]` on the line before `fn`
- To run the tests we shall use `cargo test` which will create a binary that runs the functions annotated with the `test` attribute and report whether the function passes or fails.

Let’s take a look at the following example:

```rust
#[cfg(test)] // ignore this
mod tests { // ignore this
    #[test] // this indicates the following will be a test function
    fn it_works() {
				// assertion macro that tells if 2 + 2 is equal to 4
        assert_eq!(2 + 2, 4);
    }
}

>> $ cargo test
   Compiling adder v0.1.0 (file:///projects/adder)
    Finished test [unoptimized + debuginfo] target(s) in 0.57s
     Running unittests (target/debug/deps/adder-92948b65e88960b4)

running 1 test
// here we can see that the test works fine
						// function name
test tests::**it_works ... ok**

**test result: ok.** **1 passed; 0 failed;** 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests adder

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

Let’s change the function name to see how it differs in the output:

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }
}

>> $ cargo test
   Compiling adder v0.1.0 (file:///projects/adder)
    Finished test [unoptimized + debuginfo] target(s) in 0.59s
     Running unittests (target/debug/deps/adder-92948b65e88960b4)

running 1 test
						// function name
test tests::exploration ... ok

**test result: ok. 1 passed; 0 failed**; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests adder

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

Now let’s make a test that will fail:

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn another() {
        panic!("Make this test fail");
    }
}

>> $ cargo test
   Compiling adder v0.1.0 (file:///projects/adder)
    Finished test [unoptimized + debuginfo] target(s) in 0.72s
     Running unittests (target/debug/deps/adder-92948b65e88960b4)

running 2 tests
test tests::another ... **FAILED**
test tests::exploration ... ok

failures:

---- tests::another stdout ----
thread 'main' panicked at 'Make this test fail', src/lib.rs:10:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

failures:
    tests::another

test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

error: test failed, to rerun pass '--lib'
```

### Checking Results with the assert! macro

- The `assert` macro is useful when we want to ensure that some condition in a test evaluates to `true`. If the value of assert is true the test passes, if it returns false the `assert` macro calls the `panic`  macro which causes the test to fail.

Here is another example:

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
// let's create an instance of this structure
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
		// name of our test
    fn larger_can_hold_smaller() {
				// we create 2 rectangle instances
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
    }
}

>> $ cargo test
   Compiling rectangle v0.1.0 (file:///projects/rectangle)
    Finished test [unoptimized + debuginfo] target(s) in 0.66s
     Running unittests (target/debug/deps/rectangle-6584c4561e48942e)

running 1 test
test tests::larger_can_hold_smaller ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests rectangle

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

Let’s create another test:

```rust
#[cfg(test)]
mod tests {
    use super::*;
		
		#[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(!smaller.can_hold(&larger));
    }
}

>> $ cargo test
   Compiling rectangle v0.1.0 (file:///projects/rectangle)
    Finished test [unoptimized + debuginfo] target(s) in 0.66s
     Running unittests (target/debug/deps/rectangle-6584c4561e48942e)

running 2 tests
test tests::larger_can_hold_smaller ... ok
test tests::smaller_cannot_hold_larger ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests rectangle

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

Let’s modify the can_hold function and see what happens:

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width < other.width && self.height > other.height
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(!smaller.can_hold(&larger));
    }
}

>> $ cargo test
   Compiling rectangle v0.1.0 (file:///projects/rectangle)
    Finished test [unoptimized + debuginfo] target(s) in 0.66s
     Running unittests (target/debug/deps/rectangle-6584c4561e48942e)

running 2 tests
test tests::larger_can_hold_smaller ... FAILED
test tests::smaller_cannot_hold_larger ... ok

failures:

---- tests::larger_can_hold_smaller stdout ----
thread 'main' panicked at 'assertion failed: larger.can_hold(&smaller)', src/lib.rs:28:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

failures:
    tests::larger_can_hold_smaller

test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

error: test failed, to rerun pass '--lib'
```

Here we see that only the first function passed the test and the second function failed the test.

### Testing equality with the assert_eq! and assert_ne! Macros

- A common way to test functionality is to compare the result of the code under test to the value you expect
- These two macros compare equality or inequality.

Take this example:

```rust
pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }
}

$ cargo test
   Compiling adder v0.1.0 (file:///projects/adder)
    Finished test [unoptimized + debuginfo] target(s) in 0.58s
     Running unittests (target/debug/deps/adder-92948b65e88960b4)

running 1 test
test tests::it_adds_two ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests adder

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

Let’s introduce a bug into our code now:

```rust
pub fn add_two(a: i32) -> i32 {
    a + 3
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }
}

>> $ cargo test
   Compiling adder v0.1.0 (file:///projects/adder)
    Finished test [unoptimized + debuginfo] target(s) in 0.61s
     Running unittests (target/debug/deps/adder-92948b65e88960b4)

running 1 test
test tests::it_adds_two ... FAILED

failures:

---- tests::it_adds_two stdout ----
thread 'main' panicked at 'assertion failed: `(left == right)`
  **left: `4`,
 right: `5`**', src/lib.rs:11:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

failures:
    tests::it_adds_two

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

error: test failed, to rerun pass '--lib'
```

### Adding custom failure messages

- You can also add a custom message to be printed with the failure message as optional arguments to the `assert!`, `assert_eq!`, and `assert_ne!` macros.

Consider:

```rust
pub fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(result.contains("Carol"));
    }
}
// if we run this code the test will pass
```

If we modify the function it will fail:

```rust
pub fn greeting(name: &str) -> String {
		// name is not included
    String::from("Hello!")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(result.contains("Carol"));
    }
}

>> $ cargo test
   Compiling greeter v0.1.0 (file:///projects/greeter)
    Finished test [unoptimized + debuginfo] target(s) in 0.91s
     Running unittests (target/debug/deps/greeter-170b942eb5bf5e3a)

running 1 test
test tests::greeting_contains_name ... FAILED

failures:

---- tests::greeting_contains_name stdout ----
thread 'main' panicked at 'assertion failed: result.contains(\"Carol\")', src/lib.rs:12:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

failures:
    tests::greeting_contains_name

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

error: test failed, to rerun pass '--lib'
```

If we added a more useful failure message would be like:

```rust
pub fn greeting(name: &str) -> String {
    String::from("Hello!")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        **assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was `{}`",
            result
        );**
    }
}

>> $ cargo test
   Compiling greeter v0.1.0 (file:///projects/greeter)
    Finished test [unoptimized + debuginfo] target(s) in 0.93s
     Running unittests (target/debug/deps/greeter-170b942eb5bf5e3a)

running 1 test
test tests::greeting_contains_name ... FAILED

failures:

---- tests::greeting_contains_name stdout ----
thread 'main' panicked at **'Greeting did not contain name, value was `Hello!`'**, src/lib.rs:12:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

failures:
    tests::greeting_contains_name

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

error: test failed, to rerun pass '--lib'
```

### Checking panics with should_panic

- In addition to checking that our code returns the correct values we expect, it’s also important to check that our code handles error conditions as we expect.

Consider:

```rust
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
		// We place the #[should_panic] attribute after the #[test] attribute and before the test function it applies to.
    #[should_panic]
    fn greater_than_100() {
        Guess::new(200);
    }
}

>> $ cargo test
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished test [unoptimized + debuginfo] target(s) in 0.58s
     Running unittests (target/debug/deps/guessing_game-57d70c3acb738f4d)

running 1 test
test tests::greater_than_100 - should panic ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests guessing_game

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

Let’s do some tweaks:

```rust
pub struct Guess {
    value: i32,
}

// --snip--
impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn greater_than_100() {
        Guess::new(200);
    }
}

>> $ cargo test
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished test [unoptimized + debuginfo] target(s) in 0.62s
     Running unittests (target/debug/deps/guessing_game-57d70c3acb738f4d)

running 1 test
test tests::greater_than_100 - should panic ... FAILED

failures:

---- tests::greater_than_100 stdout ----
note: test did not panic as expected

failures:
    tests::greater_than_100

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

error: test failed, to rerun pass '--lib'
```

### Using Result<T, E> in Tests

With Result<T, E> we can return an err instead of panicking:

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
						// we return ok when the test passes
            Ok(())
        } else {
						// we return Err when the tests fails with an special message included
            Err(String::from("two plus two does not equal four"))
        }
    }
}
```

## Controlling how tests are run

- Just as `cargo run` compiles your code and then runs the resulting binary, `cargo test` compiles your code in test mode and runs the resulting test binary.
- A neat feature in Rust is that you can specify command line options to change the default behavior of `cargo test`. For example, `cargo test` default behaviour is to run all the tests in parallel and capture output generated during the test, preventing the output from being displayed and compare the output with the test results.
- Running `cargo test --help` displays the options you can use with `cargo test`.

### Running tests in parallel or consecutively

- When you run multiple tests, by default they run in parallel using threads. This means the tests will finish running faster so you can get feedback quicker on whether or not your code is working.
- But make sure your tests don’t depend on each other or on any shared state, including a shared environment, such as the current working directory or environment variables.
- **For example, say each one of our tests runs a code that creates a file named *test-output.txt* and writes some data onto the txt file. Then if we test this the fist test will succeed on completing that task but the second task will fail, not because the code is incorrect but because the tests have interferred with each other while running the tests in parallel.**
- If we want to avoid running tests in parallel or if we want more control over the number of threads used, we can use `--test-threads` and the number of threads we want to use to test our binary.

Here is an example:

```rust
// We set the number of test threads to 1, telling the program not to use any parallelism.
// Running the tests using one thread will take longer than running them in parallel, but the tests won’t interfere with each other
$ cargo test -- --test-threads=1
```

### Showing function Output

- By default, if a test passes, Rust’s test library captures anything printed to standard output.
- If we call `println!` in a test and the test passes, we won’t see the `println!` output in the terminal; we’ll see only the line that indicates the test passed. If a test fails, we’ll see whatever was printed to standard output with the rest of the failure message.

Consider:

```rust
fn prints_and_returns_10(a: i32) -> i32 {
    println!("I got the value {}", a);
    10
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn this_test_will_pass() {
        let value = prints_and_returns_10(4);
        assert_eq!(10, value);
    }

    #[test]
    fn this_test_will_fail() {
        let value = prints_and_returns_10(8);
        assert_eq!(5, value);
    }
}

>> $ cargo test
   Compiling silly-function v0.1.0 (file:///projects/silly-function)
    Finished test [unoptimized + debuginfo] target(s) in 0.58s
     Running unittests (target/debug/deps/silly_function-160869f38cff9166)

running 2 tests
test tests::this_test_will_fail ... FAILED
test tests::this_test_will_pass ... ok

failures:

---- tests::this_test_will_fail stdout ----
// I got the value 4 for the test that passed
**I got the value 8 // from the failed test**
thread 'main' panicked at 'assertion failed: `(left == right)`
  left: `5`,
 right: `10`', src/lib.rs:19:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

failures:
    tests::this_test_will_fail

test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

error: test failed, to rerun pass '--lib'
```

Note that we only see the output for the failed test but not from the test that suceeded, if we want to see printed values for passing tests as well, we can tell Rust to also show the output of successful tests at the end with `--show-output`:

```rust
$ cargo test -- --show-output
```

If we run the test again then it will change the output to:

```rust
$ cargo test -- --show-output
   Compiling silly-function v0.1.0 (file:///projects/silly-function)
    Finished test [unoptimized + debuginfo] target(s) in 0.60s
     Running unittests (target/debug/deps/silly_function-160869f38cff9166)

running 2 tests
test tests::this_test_will_fail ... FAILED
test tests::this_test_will_pass ... ok

**successes:

---- tests::this_test_will_pass stdout ----
I got the value 4

successes:
    tests::this_test_will_pass

failures:

---- tests::this_test_will_fail stdout ----
I got the value 8**
thread 'main' panicked at 'assertion failed: `(left == right)`
  left: `5`,
 right: `10`', src/lib.rs:19:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

failures:
    tests::this_test_will_fail

test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

error: test failed, to rerun pass '--lib'
```

### Running a subset of tests by name

- Sometimes, running a full test suite can take a long time. If you’re working on code in a particular area, you might want to run only the tests pertaining to that code.
- You can choose which tests to run by passing `cargo test` the name or names of the test(s) you want to run as an argument.

To demonstrate this concept consider this:

```rust
pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_two_and_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn add_three_and_two() {
        assert_eq!(5, add_two(3));
    }

    #[test]
    fn one_hundred() {
        assert_eq!(102, add_two(100));
    }
}
```

If we run the tests without passing any arguments, as we saw earlier, all the tests will run in parallel:

```rust
$ cargo test
   Compiling adder v0.1.0 (file:///projects/adder)
    Finished test [unoptimized + debuginfo] target(s) in 0.62s
     Running unittests (target/debug/deps/adder-92948b65e88960b4)

running 3 tests
test tests::add_three_and_two ... ok
test tests::add_two_and_two ... ok
test tests::one_hundred ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests adder

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

### Running single tests

- We can pass the name of any test function to `cargo test` to run only that test:

```rust
$ cargo test one_hundred
   Compiling adder v0.1.0 (file:///projects/adder)
    Finished test [unoptimized + debuginfo] target(s) in 0.69s
     Running unittests (target/debug/deps/adder-92948b65e88960b4)

running 1 test
test tests::one_hundred ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; **2 filtered out**; finished in 0.00s
// 2 filtered out means we had other two tests that we didn't test in this test
```

Only the test with the name `one_hundred` ran; the other two tests didn’t match that name.

### Filtering to Run Multiple Tests

- We can specify part of a test name, for example, because two of our tests’ names contain `add`, we can run those two by running `cargo test add`:

```rust
$ cargo test add
   Compiling adder v0.1.0 (file:///projects/adder)
    Finished test [unoptimized + debuginfo] target(s) in 0.61s
     Running unittests (target/debug/deps/adder-92948b65e88960b4)

running 2 tests
test tests::add_three_and_two ... ok
test tests::add_two_and_two ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; **1 filtered out**; finished in 0.00s
// 1 filtered out means one test wasn't tested in this case i.e. one_hundred()
```

### Ignoring Some Test Unless Specifically Requested

- Sometimes a few specific tests can be very time-consuming to execute, so you might want to exclude them during most runs of `cargo test`.
- Instead of listing arguments we don’t want to run, we can label the tests that we consider time-consuming using the `ignore` attribute to exclude them, as shown here:

```rust
#[test]
fn it_works() {
    assert_eq!(2 + 2, 4);
}
// After #[test] we add the #[ignore] line to the test we want to exclude.
#[test]
#[ignore]
fn expensive_test() {
    // code that takes an hour to run
}

>> $ cargo test
   Compiling adder v0.1.0 (file:///projects/adder)
    Finished test [unoptimized + debuginfo] target(s) in 0.60s
     Running unittests (target/debug/deps/adder-92948b65e88960b4)

running 2 tests
**test expensive_test ... ignored**
test it_works ... ok

test result: ok. 1 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests adder

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

If we only want to run test with the `ignore` label, we can do so by using the following command:

```rust
$ cargo test -- --ignored
   Compiling adder v0.1.0 (file:///projects/adder)
    Finished test [unoptimized + debuginfo] target(s) in 0.61s
     Running unittests (target/debug/deps/adder-92948b65e88960b4)

running 1 test
test expensive_test ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

   Doc-tests adder

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

If you want to run all tests whether they’re ignored or not, you can run `cargo test -- --include-ignored`.

## Test Organization

- Testing is a complex discipline, and different people use different terminology and organization.
- The Rust community thinks about tests in terms of two main categories: *unit tests* and *integration tests:*
    - Unit tests are small and more focused, testing one module in isolation at a time, and can test private interfaces.
    - Integration tests are entirely external to your library and use your code in the same way any other external code would, using only the public interface and potentially exercising multiple modules per test.

### **Unit tests**

- The purpose of unit tests is to test each unit of code in isolation from the rest of the code to quickly pinpoint where code is and isn’t working as expected.
- The convention is to create a module named `tests` in each file to contain the test functions and to annotate the module with `cfg(test)`.

### The test module and #[cfg(test)]

- The `#[cfg(test)]` annotation on the tests module tells Rust to compile and run the test code only when you run `cargo test`, not when you run `cargo build`.
- Because integration tests go in a different directory, they don’t need the `#[cfg(test)]` annotation.
- The attribute `cfg` stands for *configuration* and tells Rust that the following item should only be included given a certain configuration option. In this case, the configurationoption is `test`, which is provided by Rust for compiling and running tests.

To create a project with the test module included we can write the following command:

```rust
$ cargo new adder --lib
     Created library `adder` project
$ cd adder
```

### Testing private functions

Rust’s privacy rules do allow you to test private functions, consider the following example:

```rust
pub fn add_two(a: i32) -> i32 {
    internal_adder(a, 2)
}
// Note that the internal_adder function is not marked as pub.
fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
		// we bring all of the test module’s parent’s items into scope with use super::* so we can cann internal_adder() with no issues
    use super::*;

    #[test]
    fn internal() {
        assert_eq!(4, internal_adder(2, 2));
    }
}
```

### Integration Tests

- Their purpose is to test whether many parts of your library work together correctly.
- To create integration tests, you first need a *tests* directory.

### The tests directory

- We create a *tests* directory at the top level of our project directory, next to *src.*

Take an example, within the tests directory create a integration_test.rs file and enter the following code:

```rust
use adder;

#[test]
fn it_adds_two() {
    assert_eq!(4, adder::add_two(2));
}
```

We’ve added `use adder` because we need to acknowledge that each file in the `tests` directory is a separate crate, so we need to bring our library into each test crate’s scope.

We don’t need to annotate any code in *tests/integration_test.rs* with `#[cfg(test)]`.

If we run cargo test now we will get:

```rust
$ cargo test
   Compiling adder v0.1.0 (file:///projects/adder)
    Finished test [unoptimized + debuginfo] target(s) in 1.31s
     Running unittests (target/debug/deps/adder-1082c4b063a8fbe6)

running 1 test
// internal means unit test
test tests::**internal** ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
// integration_test
     Running tests/**integration_test**.rs (target/debug/deps/integration_test-1082c4b063a8fbe6)

running 1 test
**test it_adds_two ... ok**

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
// doc tests
   **Doc-test**s adder

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

- We can still run a particular integration test function by specifying the test function’s name as an argument to `cargo test`
- To run all the tests in a particular integration test file, use the `--test` argument of `cargo test` followed by the name of the file:

```rust
$ cargo test --test integration_test
   Compiling adder v0.1.0 (file:///projects/adder)
    Finished test [unoptimized + debuginfo] target(s) in 0.64s
     Running tests/integration_test.rs (target/debug/deps/integration_test-82e7799c1bc62298)

running 1 test
test it_adds_two ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

### Summary

- Rust’s testing features provide a way to specify how code should function to ensure it continues to work as you expect, even as you make changes.
- Unit tests exercise different parts of a library separately and can test private implementation details.
- Integration tests check that many parts of the library work together correctly, and they use the library’s public API to test the code in the same way external code will use it.