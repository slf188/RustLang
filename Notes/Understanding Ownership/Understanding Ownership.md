# Understanding Ownership

- Ownership is Rust’s most unique feature
- It enables Rust to make memory safety guarantees without needing a garbage collector.

### What is ownership?

- *Ownership* is a set of rules that governs how a Rust program manages memory.
- Some languages have garbage collection that constantly looks for no-longer used memory as the program runs; in other languages, the programmer must explicitly allocate and free the memory.
- Rust uses a third approach: memory is managed through a system of ownership with a set of rules that the compiler checks. If any of the rules are violated, the program won’t compile.
- When you understand ownership, you’ll have a solid foundation for understanding the features that make Rust unique.

### The Stack and the Heap

- Many programming languages don’t require you to think about the stack and the heap very often. But in a systems programming language like Rust, whether a value is on the stack or the heap affects how the language behaves and why you have to make certain decisions.
- Both the stack and the heap are parts of memory available to your code to use at runtime, but they are structured in different ways.

**Stack**

- The stack stores values in the order it gets them and removes the values in the opposite order. This is referred to as *last in, first out*.
- Adding data is called *pushing onto the stack* and removing data is called *popping off the stack*.

![Untitled](Understand%20fe207/Untitled.png)

**Heap**

- It is less organized: when you put data on the heap, you request a certain amount of space. The memory allocator finds an empty spot in the heap that is big enough, marks it as being in use, and returns a *pointer*, which is the address of that location. This process is called allocation.

**Comparison**

- Pushing to the stack is faster than allocating on the heap. Because the location is always at the top of the stack.
- Comparatively, allocating space on the heap requires more work, because the allocator must first find a big enough space to hold the data and then perform bookkeeping to prepare for the next allocation.
- Accessing data in the heap is slower than accessing data on the stack because you have to follow a pointer to get there.

**Main idea**

- Keeping track of what parts of code are using what data on the heap, minimizing the amount of duplicate data on the heap, and cleaning up unused data on the heap so you don’t run out of space are all problems that ownership addresses

## Ownership Rules

These are the main rules:

- Each value in Rust has a variable that’s called its *owner*.
- There can only be one owner at a time.
- When the owner goes out of scope, the value will be dropped.

### Variable Scope

- A scope is the range within a program for which an item is valid. Take the following variable:

```rust
let s = "hello";
```

- The variable `s` refers to a string literal, where the value of a string is hardcoded.
- The variable is valid from the point at which it’s declared until the end of the current *scope*.

```rust
fn main() {
    {                      // s is not valid here, it’s not yet declared
        let s = "hello";   // s is valid from this point forward
        // do stuff with s
    }                      // this scope is now over, and s is no longer valid
}
```

There are two important points to make clear:

- When `s` comes *into scope*, it is valid.
- It remains valid until it goes *out of scope*.

### The `String` Type

- In order to illustrate the ideas behind ownership, we need a more complex data type than those we covered previously. The ones we have already covered are all known size, can be stored on the stack and popped off the stack, and can be quickly copied to a new independent instance.
- But in this case we want to look at data that is stored on the heap, the `String` type is the best example.
- We’ve already seen string literals, where a string value is hardcoded into our program. String literals are convenient, but they aren’t suitable for every situation.
    - One reason is that they’re immutable.
    - Another is that not every string value can be known when we write our code: for example, what if we want to take user input.
- Rust has a second string type, `String`. This type manages data allocated on the heap and as such is able to store an amount of text that is unknown to us at compile time. Here’s an example:

```rust
let s = String::from("hello");
```

The double colon `::` operator allows us to namespace this particular `from`function under the `String`type.

This kind of string can be mutated:

```rust
let mut s = String::from("hello");
s.push_str(", world!"); // push_str() appends a literal to a String
println!("{}", s); // This will print `hello, world!`
```

Why can `String` be mutated but literals cannot? The difference is how these two types deal with memory.

### Memory and Allocation

- In the case of a string literal, we know the contents at compile time, so the text is hardcoded directly into the final executable. This is why string literals are fast and efficient.
- With the `String` type, in order to support a mutable, growable piece of text, we need to allocate an amount of memory on the heap, unknown at compile time. This means:
    - The memory must be requested from the memory allocator at runtime.
    - We need a way of returning this memory to the allocator when we’re done with our `String`.
- **In languages with a *garbage collector (GC)*, the GC keeps track of and cleans up memory that isn’t being used anymore, and we don’t need to think about it. In most languages without a GC, it’s our responsibility to identify when memory is no longer being used and call code to explicitly return it, just as we did to request it. Doing this correctly has historically been a difficult programming problem. If we forget, we’ll waste memory. If we do it too early, we’ll have an invalid variable. If we do it twice, that’s a bug too.**
- Rust takes a different path: the memory is automatically returned once the variable that owns it goes out of scope.

Here’s a version of our scope example using a `String` instead of a string literal:

```rust
fn main() {
    {
        let s = String::from("hello"); // s is valid from this point forward

        // do stuff with s
    }                                // this scope is now over, and s is no longer valid
}
```

- When a variable goes out of scope, Rust calls a special function for us. This function is called `[drop](https://doc.rust-lang.org/std/ops/trait.Drop.html#tymethod.drop)`, and it’s where the author of `String` can put the code to return the memory. Rust calls `drop` automatically at the closing curly bracket.

**Note: In C++ this pattern of deallocating resources at the end of an item’s lifetime is sometimes called *Resource Acquisition Is Initialization (RAII).***

This pattern has a profound impact on the way Rust code is written.

### Ways Variables and Data interact: Move

Multiple variables can interact with the same data in different ways in Rust.

```rust
let x = 5;
let y = x;
// We now have two variables, x and y, and both equal 5.
// These two values are pushed onto the stack.
```

Now the `String` version:

```rust
let s1 = String::from("hello");
let s2 = s1;
```

This looks very similar to the previous code, we might assume that the second line would make a copy of the value in `s1` and bind it to `s2`. But this isn’t true.

A `String` is made up of 3 parts: a pointer, a length, and a capacity. This group of data is stored on the stack.

![Untitled](Understand%20fe207/Untitled%201.png)

- The pointer holds the contents of the string.
- The length is how much memory, in bytes, the contents of the `String` is currently using.
- The capacity is the total amount of memory, in bytes, that the `String` has received from the allocator.

When we assign `s1` to `s2`, the `String` data is copied, meaning we copy the pointer, the length, and the capacity that are on the stack. **We do not copy the data on the heap that the pointer refers to.**

![Untitled](Understand%20fe207/Untitled%202.png)

If Rust instead copied the heap data as well. The operation `s2 = s1`could be very expensive in terms of runtime performance if the data on the heap were large.

![Untitled](Understand%20fe207/Untitled%203.png)

**Earlier, we said that when a variable goes out of scope, Rust automatically calls the `drop`function and cleans up the heap memory for that variable. But in the previous figure shows both data pointers pointing to the same location. This is a problem: when `s2` and `s1` go out of scope, they will both try to free the same memory. This is known as a *double free* error and is one of the memory safety bugs we mentioned previously. Freeing memory twice can lead to memory corruption, which can potentially lead to security vulnerabilities.**

- To ensure memory safety, after the line `let s2 = s1`, Rust considers `s1` as no longer valid.

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1;

    println!("{}, world!", s1);
}

>> $ cargo run
   Compiling ownership v0.1.0 (file:///projects/ownership)
error[E0382]: borrow of moved value: `s1`
 --> src/main.rs:5:28
  |
2 |     let s1 = String::from("hello");
  |         -- move occurs because `s1` has type `String`, which does not implement the `Copy` trait
3 |     let s2 = s1;
  |              -- value moved here
4 | 
5 |     println!("{}, world!", s1);
  |                            ^^ value borrowed here after move

For more information about this error, try `rustc --explain E0382`.
error: could not compile `ownership` due to previous error
```

If you’ve heard the terms *shallow copy* and *deep copy* while working with other languages, the concept of copying the pointer, length, and capacity without copying the data probably sounds like making a shallow copy.

If we can illustrate what is going on, then the following figure will explain what Rust has done in order to ensure memory safety.

![Untitled](Understand%20fe207/Untitled%204.png)

That solves our problem! With only `s2` valid, when it goes out of scope, it alone will free the memory, and we’re done.

### Ways Variables and Data Interact: Clone

- If we *do* want to deeply copy the heap data of the `String`, not just the stack data, we can use a common method called `clone`.

Here’s an example of how the `clone` method in action:

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);
}
```

This works just fine and explicitly produces the behavior where the heap data *does* get copied.

**Stack-Only Data: Copy**

Here is an example of data being copied with the stack:

```rust
fn main() {
    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y);
}
```

The reason this code works is because types such as integers that have a known size at compile time are stored entirely on the stack, so copies of the actual values are quick to make. That means there’s no reason we would want to prevent `x` from being valid after we create the variable `y`.

- Rust has a special annotation called the `Copy` trait that we can place on types that are stored on the stack. If a type implements the `Copy` trait, a variable is still valid after assignment to another variable.

So what types implement the `Copy` trait? Any group of simple scalar values can implement `Copy`, and nothing that requires allocation or is some form of resource can implement `Copy`. Here are some of the types that implement `Copy`:

- All the integer types, such as `u32`.
- The Boolean type, `bool`, with values `true` and `false`.
- All the floating point types, such as `f64`.
- The character type, `char`.
- Tuples, if they only contain types that also implement `Copy`. For example, `(i32, i32)` implements `Copy`, but `(i32, String)` does not.

### Ownership and Functions

The semantics for passing a value to a function are similar to those for assigning a value to a variable. Passing a variable to a function will move or copy, just as assignment does.

```rust
fn main() {
    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here
		/* If we tried to use s after the call to takes_ownership,
		Rust would throw a compile-time error. */
    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it's okay to still
                                    // use x afterward

} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.
```

### Return Values and Scope

Returning values can also transfer ownership:

```rust
fn main() {
    let s1 = gives_ownership();         // gives_ownership moves its return
                                        // value into s1

    let s2 = String::from("hello");     // s2 comes into scope

    let s3 = takes_and_gives_back(s2);  // s2 is moved into
                                        // takes_and_gives_back, which also
                                        // moves its return value into s3
} // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
  // happens. s1 goes out of scope and is dropped.

fn gives_ownership() -> String {             // gives_ownership will move its
                                             // return value into the function
                                             // that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string                              // some_string is returned and
                                             // moves out to the calling
                                             // function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
                                                      // scope

    a_string  // a_string is returned and moves out to the calling function
}
```

When a variable that includes data on the heap goes out of scope, the value will be cleaned up by `drop` unless ownership of the data has been moved to another variable.

## References and Borrowing

Rust does let us return multiple values using a tuple, like:

```rust
fn main() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);
		// Here we have used a value and transferred ownership to the function
    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}
```

Luckily for us, Rust has a feature for using a value without transferring ownership, called *references*.

- A *reference* is like a pointer in that it’s an address we can follow to access data stored at that address that is owned by some other variable.
- Unlike a pointer, a reference is guaranteed to point to a valid value of a particular type

Here is how we would define the same `calculate_length` function that has a reference to an object as a parameter instead of taking ownership of the value:

```rust
fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
```

So a few things to notice here:

- We ass `&s1` into `calculate_length` and, in its definition, we take `&String` rather than `String`. These ampersands represent *references*, and they allow you to refer to some value without taking ownership of it.

![Screen Shot 2022-04-05 at 21.58.40.png](Understand%20fe207/Screen_Shot_2022-04-05_at_21.58.40.png)

Note: The opposite of referencing by using `&` is *dereferencing,* which is accomplished with the dereference operator, `*`.

Let’s take a closer look at the function call here:

```rust
let s1 = String::from("hello");
let len = calculate_length(&s1);
```

- The `&s1` syntax lets us create a reference that *refers* to the value of `s1` but does not own it.
- Likewise, the signature of the function uses `&` to indicate that the type of the parameter `s` is a reference.

Here is some exaplanatory comments for the function declaration:

```rust
fn calculate_length(s: &String) -> usize { // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, nothing happens.
```

- We call the action of creating a reference *borrowing*. As in real life, if a person owns something, you can borrow it from them. When you’re done, you have to give it back. You don’t own it.

But if we try to modify the variable we are borrowing then we will get the following error:

```rust
fn main() {
    let s = String::from("hello");

    change(&s);
}

fn change(some_string: &String) {
    some_string.push_str(", world");
}

»
   Compiling ownership v0.1.0 (file:///projects/ownership)
error[E0596]: cannot borrow `*some_string` as mutable, as it is behind a `&` reference
 --> src/main.rs:8:5
  |
7 | fn change(some_string: &String) {
  |                        ------- help: consider changing this to be a mutable reference: `&mut String`
8 |     some_string.push_str(", world");
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `some_string` is a `&` reference, so the data it refers to cannot be borrowed as mutable

For more information about this error, try `rustc --explain E0596`.
error: could not compile `ownership` due to previous error
```

Just as variables are immutable by default, so are references.

### Mutable References

We can fix the previous code to allow us to modify this borrowed value with just a few tweaks, that use a mutable reference:

```rust
fn main() {
    let mut s = String::from("hello");

    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
```

- First, we change `s` to be `mut`.
- Then we create a mutable reference with `&mut s` where we call the `change` function.
- Finally we update the function signature to accept a mutable reference with `some_string: &mut String`.

Mutable references have one big restriction: you can have only one mutable reference to a particular piece of data at a time. The following piece of code will crash because it violates this rule:

```rust
fn main() {
    let mut s = String::from("hello");

    let r1 = &mut s;
    let r2 = &mut s;

    println!("{}, {}", r1, r2);
}

» $ cargo run
   Compiling ownership v0.1.0 (file:///projects/ownership)
error[E0499]: cannot borrow `s` as mutable more than once at a time
 --> src/main.rs:5:14
  |
4 |     let r1 = &mut s;
  |              ------ first mutable borrow occurs here
5 |     let r2 = &mut s;
  |              ^^^^^^ second mutable borrow occurs here
6 | 
7 |     println!("{}, {}", r1, r2);
  |                        -- first borrow later used here

For more information about this error, try `rustc --explain E0499`.
error: could not compile `ownership` due to previous error
```

The benefit of having this restriction is that Rust can prevent data races at compile time. A *data race* is similar to a race condition and happens when these three behaviors occur:

- Two or more pointers access the same data at the same time.
- At least one of the pointers is being used to write to the data.
- There’s no mechanism being used to synchronize access to the data.

As always, we can use curly brackets to create a new scope, allowing for multiple mutable references, just not *simultaneous* ones:

```rust
fn main() {
    let mut s = String::from("hello");
    {
        let r1 = &mut s;
    } // r1 goes out of scope here, so we can make a new reference with no problems.

    let r2 = &mut s;
}
```

Rust enforces a similar rule for combining mutable and immutable references. This code results in an error:

```rust
fn main() {
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    let r3 = &mut s; // BIG PROBLEM

    println!("{}, {}, and {}", r1, r2, r3);
}

» $ cargo run
   Compiling ownership v0.1.0 (file:///projects/ownership)
error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
 --> src/main.rs:6:14
  |
4 |     let r1 = &s; // no problem
  |              -- immutable borrow occurs here
5 |     let r2 = &s; // no problem
6 |     let r3 = &mut s; // BIG PROBLEM
  |              ^^^^^^ mutable borrow occurs here
7 | 
8 |     println!("{}, {}, and {}", r1, r2, r3);
  |                                -- immutable borrow later used here

For more information about this error, try `rustc --explain E0502`.
error: could not compile `ownership` due to previous error
```

We *also* cannot have a mutable reference while we have an immutable one to the same value.

Instead we can first use the immutable variables and then use the mutable variables, like so:

```rust
fn main() {
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);
}
```

The scopes of the immutable references `r1` and `r2` end after the `println!` where they are last used, which is before the mutable reference `r3` is created.

### Dangling References

- A dangling pointer is a pointer that references a location in memory that may have been given to someone else--by freeing some memory while preserving a pointer to that memory.
- In Rust, by contrast, the compiler guarantees that references will never be dangling references.

Here is a demostration:

```rust
fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String {
    let s = String::from("hello");
    &s
}

» $ cargo run
   Compiling ownership v0.1.0 (file:///projects/ownership)
error[E0106]: missing lifetime specifier
 --> src/main.rs:5:16
  |
5 | fn dangle() -> &String {
  |                ^ expected named lifetime parameter
  |
  = help: this function's return type contains a borrowed value, but there is no value for it to be borrowed from
help: consider using the `'static` lifetime
  |
5 | fn dangle() -> &'static String {
  |                ~~~~~~~~

For more information about this error, try `rustc --explain E0106`.
error: could not compile `ownership` due to previous error
```

This error message mentions *lifetimes,* we’ll discuss lifetimes in later chapters, but the part we are interested is: “this function's return type contains a borrowed value, but there is no value
for it to be borrowed from”

Let’s take a closer look at the program:

```rust
fn dangle() -> &String { // dangle returns a reference to a String

    let s = String::from("hello"); // s is a new String

    &s // we return a reference to the String, s
} // Here, s goes out of scope, and is dropped. Its memory goes away.
  // Danger!
```

Because `s` is created inside `dangle`, when the code of `dangle` is finished, `s` will be deallocated.

The solution would be to do:

```rust
fn main() {
    let string = no_dangle();
}

fn no_dangle() -> String {
    let s = String::from("hello");
    s
}
```

This works without any problems. Ownership is moved out, and nothing is deallocated.

### Reference Recap

- At any given time, you can have *either* one mutable reference *or* any number of immutable references.
- References must always be valid.

## The Slice Type

- S*lices* let you reference a contiguous sequence of elements in a collection rather than the whole collection.

Here’s a challenge: write a function that takes a string and returns the first word it finds in that string. If the function doesn’t find a space in the string, the whole string must be one word, so the entire string should be returned.

```rust
fn first_word(s: &String) -> ?
```

The `first_word` function has a `&String` as a parameter. We don’t want ownership, so this is fine. But what should we return? However we could return the index of the end of the word!

```rust
fn first_word(s: &String) -> usize {
}
```

Because we need to go through the `String` element by element and check whether a value is a space, we’ll convert our `String` to an array of bytes using the `as_bytes` method:

```rust
let bytes = s.as_bytes();
```

Next, we create an iterator over the array of bytes using the `iter` method:

```rust
for (i, &item) in bytes.iter().enumerate() {
```

Inside the `for` loop, we search for the byte that represents the space by using the byte literal syntax. If we find a space, we return the position. Otherwise we return the lenght of the string:

```rust
				if item == b' ' {
            return i;
        }
    }

    s.len()
```

We now have a way to find out the index of the end of the first word in the string, but there’s a problem. We’re returning a `usize` on its own, but it’s only a meaningful number in the context of the `&String`. **In other words, because it’s a separate value from the `String`, there’s no guarantee that it will still be valid in the future.** Consider the following:

```rust
fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s); // word will get the value 5

    s.clear(); // this empties the String, making it equal to ""

    // word still has the value 5 here, but there's no more string that
    // we could meaningfully use the value 5 with. word is now totally invalid!
}
```

Luckily, Rust has a solution to this problem: string slices.

### String Slices

A string slice is a reference to part of a String:

```rust
let s = String::from("hello world");
let hello = &s[0..5];
let world = &s[6..11];
```

Rather than a reference to the entire `String`, `hello` is a reference to a portion of the `String`, specified in the extra `[0..5]` bit. We create slices using a range within brackets by specifying `[starting_index..ending_index]`.

![Screen Shot 2022-04-05 at 22.53.53.png](Understand%20fe207/Screen_Shot_2022-04-05_at_22.53.53.png)

With Rust’s `..` range syntax, if you want to start at index zero, you can drop the value before the two periods. In other words, these are equal:

```rust
let s = String::from("hello");
// These two variables are equal
let slice = &s[0..2];
let slice = &s[..2];
```

If your slice includes the last byte of the `String`, you can drop the trailing number.

```rust
let s = String::from("hello");
let len = s.len();
// These two are equal
let slice = &s[3..len];
let slice = &s[3..];
```

You can also drop both values to take a slice of the entire string:

```rust
let s = String::from("hello");
let len = s.len();
// These two are equal
let slice = &s[0..len];
let slice = &s[..];
```

So if we rewrite the `first_word` function with slices we will get:

```rust
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
```

Here we get the index of the end of the word in the same way by looking for the first occurrence of space. When we find a space, we return a string slice using the start of the string and the index of the space.

But if we run the program as it is and try to clear the string, we will get the following error:

```rust
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s);

    s.clear(); // error!

    println!("the first word is: {}", word);
}

» $ cargo run
   Compiling ownership v0.1.0 (file:///projects/ownership)
error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
  --> src/main.rs:18:5
   |
16 |     let word = first_word(&s);
   |                           -- immutable borrow occurs here
17 | 
18 |     s.clear(); // error!
   |     ^^^^^^^^^ mutable borrow occurs here
19 | 
20 |     println!("the first word is: {}", word);
   |                                       ---- immutable borrow later used here

For more information about this error, try `rustc --explain E0502`.
error: could not compile `ownership` due to previous error
```

Because `clear` needs to truncate the `String`, it needs to get a mutable reference. The `println!` after the call to `clear` uses the reference in `word`, so the immutable reference must still be active at that point.

### Other Slices

String slices, as you might imagine, are specific to strings. But there’s a more general slice type, too. Consider this array, just as we might want to refer to a part of a string, we might want to refer to part of an array. 

```rust
let a = [1, 2, 3, 4, 5];
let slice = &a[1..3];
```

## Summary

- The concepts of ownership, borrowing, and slices ensure memory safety in Rust programs at compile time.
- The Rust language gives you control over your memory usage in the same way as other systems programming languages, but having the owner of data automatically clean up that data when the owner goes out of scope means you don’t have to write and debug extra code to get this control.