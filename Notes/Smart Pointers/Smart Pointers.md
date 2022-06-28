# Smart Pointers

- A pointer is a concept for a variable that contains an address in memory. This address refers to or points to other data.
- **The most common kind of pointer in Rust is a Reference!**
- References are indicated by the `&` symbol and borrow the value they point to. References do not have any other special ability except for referring to data.
- Smart pointers on the other hand are data structures that do not only act like a pointer but also have additional metada and capabilities.
- The concept of smart pointers isn’t unique to Rust: smart pointers originated in C++ and other existing programming langauges.
- There are a variety of smart pointers in Rust defined within the standard library and provide functionality beyond providing reference, one example is *reference counting smart pointer*. This type of pointer allows us to have multiple owners of data by keeping track of the number of owners, and when no owners remain we clean up the data.
- A few smart pointers we’ve already covered are: `String`, `Vec<T>` although we did not call them smart pointers. They both count as smart pointers because they own some memory, and allows us to manipulate it, and they also contain other metadata.
- Smart pointers are usually implemented using `Structs`.
- What differs from a ordinary pointer and a smart pointer is:
    - Smart pointers implement the `Deref` and `Drop` traits.
        - Deref allows and instance of the struct to behave like a reference.
        - Drop allows us to customize the code that is run when an instance of the instance goes out of scope.

In this chapter we will only review the most common type of smart pointers available within the standard library:

- `Box<T>` for allocating values in heap.
- `Rc<T>` reference counting type that enables multiple ownership.
- `Ref<T>` and `RefMut<T>` accessed through `RefCell<T>` a type that enforces borrowing rules at runtime instead of compile time.

In addition we will also cover *interior mutability* where an immutable type exposes API for mutating an interior value. And also *reference cycles* how they can leak memory and how to prevent them.

## Using Box<T> to point data on the heap

- The most straightforward smart pointer is a box, whose type is written as Box<T>.
- Box allows us to store data on the heap rather than the stack. The only remaining data in the stack is the pointer referring to the heap data.

We will use the box smart pointer in the following common situations:

- When we have a type whose size can’t be known at compile time.
- When we have a large amount of data and we want to transfer ownership without unnecesarily copying data when transfering.
- When we want to own a value.

### Using Box<T> to store data on the heap

Let’s showcase how we can use Box<T> to store an i32 value on the heap:

```rust
fn main() {
		// b points to the value 5 allocated in heap
    let b = Box::new(5);
    println!("b = {}", b);
}

» b = 5
```

- Putting single values on the heap isn’t very helpful, having a single value like i32 on the stack, where they are stored by default, is more appropiate in majority of situations

### Enabling recursive types with boxes

- One type whose size can’t be known at compile time is a recursive type.
- We are now going to be using the cons list, a data type common in functional programming languages

<aside>
💡 A cons list is a data structure that comes from the lisp programming language. Cons (short for “construct function”) constructs a new pair from its two arguments. Usually a single pair and another pair. These pairs form a list. Each item in a cons list contains two elements: the value of the current item and the next item. The last value in the list contains a value called Nil

</aside>

- Cons isn’t a commonly used data structure within Rust. Most of the time when we will be dealing with a list of items in Rust it is better to use the `Vec<T>` type.

For example purposes we will use the cons data structure:

 

```rust
enum List {
    Cons(i32, List),
    Nil,
}
```

Using the `List` type to store the list 1, 2, 3 it would look the following way:

```rust
use crate::List::{Cons, Nil};

fn main() {
		// the first value 1 holds another list and so on...
		// the final value most always be nil, it signals the end of the list.
    let list = Cons(1, Cons(2, Cons(3, Nil)));
}
```

If we compile the code:

```rust
» $ cargo run
   Compiling cons-list v0.1.0 (file:///projects/cons-list)
error[E0072]: recursive type `List` has infinite size
 --> src/main.rs:1:1
  |
1 | enum List {
  | ^^^^^^^^^ recursive type has infinite size
2 |     Cons(i32, List),
  |               ---- recursive without indirection
  |
help: insert some indirection (e.g., a `Box`, `Rc`, or `&`) to make `List` representable
  |
2 |     Cons(i32, Box<List>),
  |               ++++    +

error[E0391]: cycle detected when computing drop-check constraints for `List`
 --> src/main.rs:1:1
  |
1 | enum List {
  | ^^^^^^^^^
  |
  = note: ...which immediately requires computing drop-check constraints for `List` again
  = note: cycle used when computing dropck types for `Canonical { max_universe: U0, variables: [], value: ParamEnvAnd { param_env: ParamEnv { caller_bounds: [], reveal: UserFacing }, value: List } }`

Some errors have detailed explanations: E0072, E0391.
For more information about an error, try `rustc --explain E0072`.
error: could not compile `cons-list` due to 2 previous errors
```

The error shows this type has infinite size, the reason is we defined `List` to be recursive: it holds another value of itself directly. As a result Rust can’t figure out how much space it needs to store the list value.

### Computing the size of a non recursive type

Recall this enum we defined previously:

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
```

If we compare what happens when Rust tries to determine how much space a recursive type like list needs. The compiler starts by looking at the Cons variant which holds multiple values of type i32. This process will continue infinitely as shown in the following figure:

![Untitled](Smart%20Pointers%20f0c273517a214dbabfb3f0b03c3efef2/Untitled.png)

### Using Box<T> to get a recursive type with a known size

Rust is unable to figure how much space is needed to allocate for recursively defined types, so the compiler includes the following suggestion:

```rust
help: insert some indirection (e.g., a `Box`, `Rc`, or `&`) to make `List` representable
  |
2 |     **Cons(i32, Box<List>)**,
  |               ^^^^    ^
```

In this suggestion, “indirection” means that instead of directly storing values, we can store the value indirectly by storing a pointer to the value instead.

- Because a `Box<T>` is a pointer, Rust always knows how much space a `Box<T>` needs: a pointer’s size doesn’t change based on the amount of data it’s pointing to.

Let’s make the following tweak in our code:

```rust
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}
```

- By using a box, we’ve broken the infinite, recursive chain, so the compiler can figure out the size it needs to store a `List` value.

![Untitled](Smart%20Pointers%20f0c273517a214dbabfb3f0b03c3efef2/Untitled%201.png)

## Treating smart pointers like regular references with the `Deref` trait

- Implementing the `Deref` trait allows you to customize the behavior of the *dereference operator*, `*`

### Following the pointer to the value with the dereference operator

- A regular reference is a type of pointer, and one way to think of a pointer is as an arrow to a value stored somewhere else.

Here is an example of how we create a reference to an i32 value and then use the dereference operator to follow the reference to the data:

```rust
fn main() {
    let x = 5;
		// we set y equal to the reference to x
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);
}

$ cargo run
   Compiling deref-example v0.1.0 (file:///projects/deref-example)
error[E0277]: can't compare `{integer}` with `&{integer}`
 --> src/main.rs:6:5
  |
6 |     assert_eq!(5, y);
  |     ^^^^^^^^^^^^^^^^ no implementation for `{integer} == &{integer}`
  |
  = help: the trait `PartialEq<&{integer}>` is not implemented for `{integer}`
  = note: this error originates in the macro `assert_eq` (in Nightly builds, run with -Z macro-backtrace for more info)

For more information about this error, try `rustc --explain E0277`.
error: could not compile `deref-example` due to previous error
```

Comparing a number and a reference to a number isn’t allowed because they’re different types. We must use the dereference operator to follow the reference to the value it’s pointing to.

### Using Box<T> like a reference

- We can write the same code from above to use Box<T> instead of a reference, the dereference operator will work as well

```rust
fn main() {
    let x = 5;
		// set y to be an instance of a box pointing a copied value of x
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}
```

### Defining our own smart pointer

- Let’s define something similar to Box<T> along with a new function defined on Box<T>

```rust
// MyBox is the new struct that is declared with a new generic parameter T
struct MyBox<T>(T);

impl<T> MyBox<T> {
		// new takes one parameter of type T and returns the struct instance that holds the value passed
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

fn main() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}

$ cargo run
   Compiling deref-example v0.1.0 (file:///projects/deref-example)
error[E0614]: type `MyBox<{integer}>` cannot be dereferenced
  --> src/main.rs:14:19
   |
14 |     assert_eq!(5, *y);
   |                   ^^

For more information about this error, try `rustc --explain E0614`.
error: could not compile `deref-example` due to previous error
```

- Our `MyBox<T>` type can’t be dereferenced because we haven’t implemented that ability on our type.

### Treating a type like a reference by implementing the deref trait

- The `Deref` trait, provided by the standard library, requires us to implement one method named `deref` that borrows `self` and returns a reference to the inner data.

```rust
use std::ops::Deref;

impl<T> Deref for MyBox<T> {
		// define an associated type for the Deref trait to use
    type Target = T;
		// deref returns a reference to the value we want to access with the * operator
		// we take a value of any type that implements Deref and call the deref method to get a & reference that it knows how to dereference.
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
```

### Implicit Deref Coercions with functions and methods

- *Deref coercion*is a convenience that Rust performs on arguments to functions and methods.
- Deref coercion works only on types that implement the `Deref` trait.
- Deref coercion converts a reference to such a type into a reference to another type.
- The deref coercion feature also lets us write more code that can work for either references or smart pointers.

```rust
fn hello(name: &str) {
    println!("Hello, {}!", name);
}

fn main() {
    let m = MyBox::new(String::from("Rust"));
		// calling the hello function with the argument &m, which is a reference to a MyBox<String> value.
    hello(&m);
}
```

- The code without deref coercions is harder to read, write, and understand with all of these symbols involved.

Without deref coercions it would be:

```rust
fn main() {
    let m = MyBox::new(String::from("Rust"));
    hello(&(*m)[..]);
}
```

### How Deref Coercion interacts with mutability

- We can use `DerefMut` trait to override the `*` operator on mutable references.

## Running code on cleanup with the drop trait

- The second trait important to the smart pointer pattern is `Drop`, which lets you customize what happens when a value is about to go out of scope.
- In some languages, the programmer must call code to free memory or resources every time they finish using an instance of a smart pointer. If they forget, the system might become overloaded and crash. In Rust, you can specify that a particular bit of code be run whenever a value goes out of scope, and the compiler will insert this code automatically.

To see when Rust calls `drop`, let’s implement `drop` with `println!` statements for now:

```rust
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
		// 2 instances of customsmartpointer and print the customsmartpointers
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created.");
}

// the instances will go out of scope at the end of main

$ cargo run
CustomSmartPointers created.
Dropping CustomSmartPointer with data `other stuff`!
Dropping CustomSmartPointer with data `my stuff`!
```

### Dropping a value early with `std::mem::drop`

Rust doesn’t let you call the `Drop` trait’s `drop` method manually; instead you have to call the `std::mem::drop` function provided by the standard library if you want to force a value to be dropped before the end of its scope.

If we try to call the `Drop` trait’s `drop` method manually, we will get a compiler error:

```rust
fn main() {
    let c = CustomSmartPointer {
        data: String::from("some data"),
    };
    println!("CustomSmartPointer created.");
    c.drop();
    println!("CustomSmartPointer dropped before the end of main.");
}

$ cargo run
   Compiling drop-example v0.1.0 (file:///projects/drop-example)
error[E0040]: explicit use of destructor method
  --> src/main.rs:16:7
   |
16 |     c.drop();
   |     --^^^^--
   |     | |
   |     | **explicit destructor calls not allowed**
   |     help: consider using `drop` function: `drop(c)`

For more information about this error, try `rustc --explain E0040`.
error: could not compile `drop-example` due to previous error
```

Here is some explanation of the error message we just got:

- This error message states that we’re not allowed to explicitly call `drop`.
- The `drop` function in Rust is one particular destructor.

A solution to this error would be to use a different type of function, the `std::mem::drop` function is different from the `drop` method in the `Drop` trait.

```rust
fn main() {
    let c = CustomSmartPointer {
        data: String::from("some data"),
    };
    println!("CustomSmartPointer created.");
    drop(c);
    println!("CustomSmartPointer dropped before the end of main.");
}

$ cargo run
   Compiling drop-example v0.1.0 (file:///projects/drop-example)
    Finished dev [unoptimized + debuginfo] target(s) in 0.73s
     Running `target/debug/drop-example`
CustomSmartPointer created.
Dropping CustomSmartPointer with data `some data`!
CustomSmartPointer dropped before the end of main.
```

Now that we’ve examined `Box<T>` and some of the characteristics of smart pointers, let’s look at a few other smart pointers defined in the standard library.

## `Rc<T>`, the Reference Counted Smart Pointer

- In the majority of cases: you know exactly which variable owns a given value. However there are cases when a single value might have multiple owners.
- For example in the cases of graph data structures, multiple edges point to the same node, and the node is conceptually owned by all the edges that point to it.
- To enable multiple ownership, Rust has a type called `Rc<T>` which is an abbreviation of reference counting. It keeps track of the number of references a value is still in use.
- Imagine `Rc<T>` as a TV in a family room. When one person enters to watch TV, they turn it on. Others can come into the room and watch the TV. When the last person leaves the room, they turn off the TV because it’s no longer being used. If someone turns off the TV while others are still watching it, there would be uproar from the remaining TV watchers!

### Using `Rc<T>` to share data

We’ll create list `a` that contains 5 and then 10. Then we’ll make two more lists: `b` that starts with 3 and `c` that starts with 4. Both `b` and `c` lists will then continue on to the first `a` list containing 5 and 10:

![Screen Shot 2022-06-28 at 16.29.37.png](Smart%20Pointers%20f0c273517a214dbabfb3f0b03c3efef2/Screen_Shot_2022-06-28_at_16.29.37.png)

If we try to implement this scenario using `List` with `Box<T>` it won’t work:

```rust
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
    let b = Cons(3, Box::new(a));
    let c = Cons(4, Box::new(a));
}

$ cargo run
   Compiling cons-list v0.1.0 (file:///projects/cons-list)
error[E0382]: use of moved value: `a`
  --> src/main.rs:11:30
   |
9  |     let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
   |         - move occurs because `a` has type `List`, which does not implement the `Copy` trait
10 |     let b = Cons(3, Box::new(a));
   |                              - value moved here
11 |     let c = Cons(4, Box::new(a));
   |                              ^ value used here after move

For more information about this error, try `rustc --explain E0382`.
error: could not compile `cons-list` due to previous error
```

If we try to implement this same behaviour with Rc<T> it will be:

```rust
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
// we bring Rc into scope 
use std::rc::Rc;

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    let b = Cons(3, Rc::clone(&a));
    let c = Cons(4, Rc::clone(&a));
}
```

### Cloning an Rc<T> increases the reference count

Let us change `main` so it has an inner scope around list `c`; then we can see how the reference count changes when `c` goes out of scope.

```rust
fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}

$ cargo run
   Compiling cons-list v0.1.0 (file:///projects/cons-list)
    Finished dev [unoptimized + debuginfo] target(s) in 0.45s
     Running `target/debug/cons-list`
count after creating a = 1
count after creating b = 2
count after creating c = 3
count after c goes out of scope = 2
```

At each point in the program where the reference count changes, we print the reference count

## `RefCell<T>` and the interior mutability pattern

- *Interior mutability* is a design pattern in Rust that allows you to mutate data even when there are immutable references to that data; normally, this action is disallowed by the borrowing rules.

### Enforcing borrowing rules at runtime with `RefCell<T>`

- The `RefCell<T>` type represents single ownership over the data it holds.
- The `RefCell<T>` type is useful when you’re sure your code follows the borrowing rules but the compiler is unable to understand and guarantee that.

Here is a recap of the reasons to choose `Box<T>`, `Rc<T>`, or `RefCell<T>`:

- `Rc<T>` enables multiple owners of the same data; `Box<T>` and `RefCell<T>` have single owners.
- `Box<T>` allows immutable or mutable borrows checked at compile time; `Rc<T>` allows only immutable borrows checked at compile time; `RefCell<T>` allows immutable or mutable borrows checked at runtime.
- Because `RefCell<T>` allows mutable borrows checked at runtime, you can mutate the value inside the `RefCell<T>` even when the `RefCell<T>` is immutable.

### Interior mutability: a mutable borrow to an immutable value

- A consequence of the borrowing rules is that when you have an immutable value, you can’t borrow it mutably.

As an example, this code won’t compile:

```rust
fn main() {
    let x = 5;
    let y = &mut x;
}

$ cargo run
   Compiling borrowing v0.1.0 (file:///projects/borrowing)
error[E0596]: cannot borrow `x` as mutable, as it is not declared as mutable
 --> src/main.rs:3:13
  |
2 |     let x = 5;
  |         - help: consider changing this to be mutable: `mut x`
3 |     let y = &mut x;
  |             ^^^^^^ cannot borrow as mutable

For more information about this error, try `rustc --explain E0596`.
error: could not compile `borrowing` due to previous error
```

### A use case for interior mutability: Mock Objects

- A *test double* is the general programming concept for a type used in place of another type during testing.
- *Mock objects* are specific types of test doubles that record what happens during a test so you can assert that the correct actions took place.

Here’s the scenario we’ll test: we’ll create a library that tracks a value against a maximum value and sends messages based on how close to the maximum value the current value is.

```rust
pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger
                .send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger
                .send("Warning: You've used up over 75% of your quota!");
        }
    }
}
```

- The `Messenger` trait has one method called `send` that takes an immutable reference to `self` and the text of the message.
- The other important part is that we want to test the behavior of the `set_value` method on the `LimitTracker`.
- We need a mock object that, instead of sending an email or text message when we call `send`, will only keep track of the messages it’s told to send.

### Having multiple owners of mutable data by combining Rc<T> and RefCell<T>

- A common way to use `RefCell<T>` is in combination with `Rc<T>`
- `Rc<T>`lets you have multiple owners of some data, but it only gives immutable access to that data.
- If you have an `Rc<T>` that holds a `RefCell<T>`, you can get a value that can have multiple owners *and* that you can mutate!

Here is a quick example:

```rust
#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}

$ cargo run
   Compiling cons-list v0.1.0 (file:///projects/cons-list)
    Finished dev [unoptimized + debuginfo] target(s) in 0.63s
     Running `target/debug/cons-list`
a after = Cons(RefCell { value: 15 }, Nil)
b after = Cons(RefCell { value: 3 }, Cons(RefCell { value: 15 }, Nil))
c after = Cons(RefCell { value: 4 }, Cons(RefCell { value: 15 }, Nil))
```

Here is a quick explanation of the code:

- We create a value that is an instance of `Rc<RefCell<i32>>` and store it in a variable named `value` so we can access it directly later.
- Then we create a `List` in `a` with a `Cons` variant that holds `value`. We need to clone `value` so both `a` and `value` have ownership of the inner `5` value rather than transferring ownership from `value` to `a` or having `a` borrow from `value`.
- We wrap the list `a` in an `Rc<T>` so when we create lists `b` and `c`, they can both refer to `a`
- After we’ve created the lists in `a`, `b`, and `c`, we add 10 to the value in `value`.
- When we print `a`, `b`, and `c`, we can see that they all have the modified value of 15 rather than 5:

## Reference Cycles Can Leak Memory

- Rust’s memory safety guarantees make it difficult, but not impossible, to accidentally create memory that is never cleaned up (known as a *memory leak*).
- Preventing memory leaks entirely is not one of Rust’s guarantees, meaning memory leaks are memory safe in Rust.

### Creating a reference cycle

Let’s look at how a reference cycle might happen and how to prevent it in the following way:

```rust
use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

fn main() {}
```

Now here this code creates a list in `a` and a list in `b` that points to the list in `a`. Then it modifies the list in `a` to point to `b`, creating a reference cycle. There are `println!` statements along the way to show what the reference counts are at various points in this process.

```rust
use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

fn main() {
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));

    // Uncomment the next line to see that we have a cycle;
    // it will overflow the stack
    // println!("a next item = {:?}", a.tail());
}

$ cargo run
a initial rc count = 1
a next item = Some(RefCell { value: Nil })
a rc count after b creation = 2
b initial rc count = 1
b next item = Some(RefCell { value: Cons(5, RefCell { value: Nil }) })
b rc count after changing a = 2
a rc count after changing a = 2
```

We create an `Rc<List>` instance holding a `List` value in the variable `a` with an initial list of `5, Nil`. We then create an `Rc<List>` instance holding another `List` value in the variable `b` that contains the value 10 and points to the list in `a`.

To visualize this reference cycle here is a diagram:

![Screen Shot 2022-06-28 at 17.03.10.png](Smart%20Pointers%20f0c273517a214dbabfb3f0b03c3efef2/Screen_Shot_2022-06-28_at_17.03.10.png)

### Preventing Reference cycles: turning an Rc<T> into a Weak<T>

- Calling `Rc::clone` increases the `strong_count` of an `Rc<T>` instance, and an `Rc<T>` instance is only cleaned up if its `strong_count` is 0.
- You can also create a *weak reference* to the value within an `Rc<T>` instance by calling `Rc::downgrade` and passing a reference to the `Rc<T>`.
- Strong references are how you can share ownership of an `Rc<T>` instance. Weak references don’t express an ownership relationship.

### Creating a tree data structure: a node with child nodes

To start, we’ll build a tree with nodes that know about their child nodes. We’ll create a struct named `Node` that holds its own `i32` value as well as references to its children `Node` values:

```rust
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
struct Node {
    value: i32,
    children: RefCell<Vec<Rc<Node>>>,
}
```

- We want a `Node` to own its children, and we want to share that ownership with variables so we can access each `Node` in the tree directly.
- We also want to modify which nodes are children of another node, so we have a `RefCell<T>` in `children` around the `Vec<Rc<Node>>`.
- Next, we’ll use our struct definition and create one `Node` instance named `leaf` with the value 3 and no children, and another instance named `branch` with the value 5 and `leaf` as one of its children.

```rust
fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        children: RefCell::new(vec![]),
    });

    let branch = Rc::new(Node {
        value: 5,
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });
}
```

### Adding a reference from a child to its parent

- To make the child node aware of its parent, we need to add a `parent` field to our `Node` struct definition.
- Thinking about the relationships another way, a parent node should own its children: if a parent node is dropped, its child nodes should be dropped as well. However, a child should not own its parent: if we drop a child node, the parent should still exist.
- So instead of `Rc<T>`, we’ll make the type of `parent` use `Weak<T>`, specifically a `RefCell<Weak<Node>>`.

```rust
use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
}
```

### Visualizing changes to strong_count and weak_count

Let’s look at how the `strong_count` and `weak_count` values of the `Rc<Node>` instances change by creating a new inner scope and moving the creation of `branch` into that scope. The modifications are shown below:

```rust
fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );

    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!(
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch),
        );

        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );
    }

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );
}
```

### Summary

- This chapter covered how to use smart pointers to make different guarantees and trade-offs from those Rust makes by default with regular references.
- The `Box<T>` type has a known size and points to data allocated on the heap.
- The `Rc<T>` type keeps track of the number of references to data on the heap so that data can have multiple owners.
- The `RefCell<T>` type with its interior mutability gives us a type that we can use when we need an immutable type but need to change an inner value of that type; it also enforces the borrowing rules at runtime instead of at compile time.

If this chapter has piqued your interest and you want to implement your own smart pointers, check out [“The Rustonomicon”](https://doc.rust-lang.org/stable/nomicon/index.html)
 for more useful information.