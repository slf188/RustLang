# Managing Growing Projects with Packages, Crates and Modules

- Organizing your code will be important because keeping track of your entire program in your head will become impossible.
- By grouping related functionality and separating code with distinct features, we can achieve such crucial organization within our code.
- As a project grows, you can organize code by splitting it into multiple modules and then multiple files.
- A package can contain multiple binary crates and optionally one library crate. As a package grows, you can extract parts into separate crates that become external dependencies.
- In addition to grouping functionality, encapsulating implementation details lets you reuse code at a higher level.
- A related concept is scope: the nested context in which code is written has a set of names that are defined as “in scope.”
- Rust has a number of features that allow you to manage your code’s organization, including which details are exposed, which details are private, and what names are in each scope in your programs. These features, sometimes collectively referred to as the *module system*, include:
    - **Packages:** A Cargo feature that lets you build, test, and share crates
    - **Crates:** A tree of modules that produces a library or executable
    - **Modules** and **use:** Let you control the organization, scope, and privacy of paths
    - **Paths:** A way of naming an item, such as a struct, function, or module

Let’s cover all these features, by the end, we should achieve a solid understanding of how the module system and work with the scope feature like a pro.

## Packages and Crates

- The first parts of the module system we’ll cover are packages and crates.
- A crate is a binary or library.
- A *package* is one or more crates that provide a set of functionality. A package contains a *Cargo.toml* file that describes how to build those crates.
- A package can contain at most one library crate. It can contain as many binary crates as you’d like, but it must contain at least one crate (either library or binary).

Let’s walk through what happens when we create a package. First, we enter the command `cargo new`:

```bash
$ cargo new my-project
     Created binary (application) `my-project` package
$ ls my-project
Cargo.toml
src
$ ls my-project/src
main.rs
```

- Cargo created a *Cargo.toml* file, giving us a package.
- Looking at the contents of *Cargo.toml*, there’s no mention of *src/main.rs* because Cargo follows a convention that *src/main.rs* is the crate root of a binary crate with the same name as the package.
- Cargo passes the crate root files to `rustc` to build the library or binary.
- Here, we have a package that only contains *src/main.rs*, meaning it only contains a binary crate named `my-project`. If a package contains *src/main.rs* and *src/lib.rs*, it has two crates: a binary and a library, both with the same name as the package.
- A crate will group related functionality together in a scope so the functionality is easy to share between multiple projects.
- For example, the `rand` crate we used in [Chapter 2](https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html#generating-a-random-number) provides functionality that generates random numbers. We can use that functionality in our own projects by bringing the `rand` crate into our project’s scope.
- Keeping a crate’s functionality in its own scope clarifies whether particular functionality is defined in our crate or the `rand` crate and prevents potential conflicts. For example, the `rand` crate provides a trait named `Rng`. We can also define a `struct` named `Rng` in our own crate. Because a crate’s functionality is namespaced in its own scope, when we add `rand` as a dependency, the compiler isn’t confused about what the name `Rng` refers to. In our crate, it refers to the `struct Rng` that we defined. We would access the `Rng` trait from the `rand` crate as `rand::Rng`.

## Defining Modules to Control Scope and Privacy

- *Modules* let us organize code within a crate into groups for readability and easy reuse. Modules also control the *privacy* of items, which is whether an item can be used by outside code (*public*) or is an internal implementation detail and not available for outside use (*private*).

As an example, let’s write a library crate that provides the functionality of a restaurant.

- In the restaurant industry, some parts of a restaurant are referred to as *front of house* and others as *back of house*. Front of house is where customers are; this is where hosts seat customers, servers take orders and payment, and bartenders make drinks. Back of house is where the chefs and cooks work in the kitchen, dishwashers clean up, and managers do administrative work.

To structure our crate in the same way that a real restaurant works, we can organize the functions into nested modules. Create a new library named `restaurant` by running `cargo new --lib restaurant`; then put the code into *src/lib.rs* to define some modules and function signatures.

```rust
// We define a module by starting with the mod keyword and then specify the name of the module (in this case, front_of_house)
mod front_of_house {
    // we can have other modules, as in this case with the modules hosting and serving
		mod hosting {
				// Modules can also hold definitions for other items, such as structs, enums, constants, traits or in this case functions
        fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}
```

- By using modules, we can group related definitions together and name why they’re related. Programmers using this code would have an easier time finding the definitions they wanted to use because they could navigate the code based on the groups rather than having to read through all the definitions.
- Earlier, we mentioned that *src/main.rs* and *src/lib.rs* are called crate roots. The reason for their name is that the contents of either of these two files form a module named `crate` at the root of the crate’s module structure, known as the *module tree.*

The module tree of front_of_house module is the following:

```rust
crate
 └── front_of_house
     ├── hosting
     │   ├── add_to_waitlist
     │   └── seat_at_table
     └── serving
         ├── take_order
         ├── serve_order
         └── take_payment
```

- This tree shows how some of the modules nest inside one another (for example, `hosting` nests inside `front_of_house`). The tree also shows that some modules are *siblings* to each other, meaning they’re defined in the same module.
    - If module A is contained inside module B, we say that module A is the *child* of module B and that module B is the *parent* of module A.
    - Notice that the entire module tree is rooted under the implicit module named `crate`.
    - The module tree might remind you of the filesystem’s directory tree on your computer; this is a very apt comparison!

## Paths for Referring to an Item in the Module Tree

- To show Rust where to find an item in a module tree, we use a path in the same way we use a path when navigating a filesystem. If we want to call a function, we need to know its path.

A path can take two forms:

• An *absolute path* starts from a crate root by using a crate name or a literal `crate`.

• A *relative path* starts from the current module and uses `self`, `super`, or an identifier in the current module.

Both absolute and relative paths are followed by one or more identifiers separated by double colons (`::`).

Let us show how we can call a function using an absolue and relative path:

```rust
mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}
    }
}
// The eat_at_restaurant function is part of our library crate’s public API, so we mark it with the pub keyword.
pub fn eat_at_restaurant() {
		// The add_to_waitlist function is defined in the same crate as eat_at_restaurant, which means we can use the crate keyword to start an absolute path.
		// After crate, we include each of the successive modules until we make our way to add_to_waitlist.
		// You can imagine a filesystem with the same structure, and we’d specify the path /front_of_house/hosting/add_to_waitlist
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();
    
		// The path starts with front_of_house, the name of the module defined at the same level of the module tree as eat_at_restaurant.
		// Starting with a name means that the path is relative.
		// Relative path
    front_of_house::hosting::add_to_waitlist();
		// Our preference is to specify absolute paths because it’s more likely to move code definitions and item calls independently of each other.
}

>> $ cargo build
   Compiling restaurant v0.1.0 (file:///projects/restaurant)
error[E0603]: module `hosting` is private
 --> src/lib.rs:9:28
  |
9 |     crate::front_of_house::hosting::add_to_waitlist();
  |                            ^^^^^^^ private module
  |
note: the module `hosting` is defined here
 --> src/lib.rs:2:5
  |
2 |     mod hosting {
  |     ^^^^^^^^^^^

error[E0603]: module `hosting` is private
  --> src/lib.rs:12:21
   |
12 |     front_of_house::hosting::add_to_waitlist();
   |                     ^^^^^^^ private module
   |
note: the module `hosting` is defined here
  --> src/lib.rs:2:5
   |
2  |     mod hosting {
   |     ^^^^^^^^^^^

For more information about this error, try `rustc --explain E0603`.
error: could not compile `restaurant` due to 2 previous errors
```

In the error message the compiler states that it won’t let us use neither hosting and add_to_waitlist() because it doesn’t have access to the private sections.

- Modules aren’t useful only for organizing your code. They also define Rust’s *privacy boundary*: the line that encapsulates the implementation details external code isn’t allowed to know about, call, or rely on. So, if you want to make an item like a function or struct private, you put it in a module.
- The way privacy works in Rust is that all items (functions, methods, structs, enums, modules, and constants) are private by default. To continue with the restaurant metaphor, think of the privacy rules as being like the back office of a restaurant: what goes on in there is private to restaurant customers, but office managers can see and do everything in the restaurant in which they operate.
- But you can expose inner parts of child modules’ code to outer ancestor modules by using the `pub` keyword to make an item public.

### Exposing Paths with the `pub` keyword

Let us use the `pub` keyword to make the hosting module public and see if this time `eat_at_restaurant` has access to it:

```rust
mod front_of_house {
    pub mod hosting {
        fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}

>> $ cargo build
   Compiling restaurant v0.1.0 (file:///projects/restaurant)
error[E0603]: function `add_to_waitlist` is private
 --> src/lib.rs:9:37
  |
9 |     crate::front_of_house::hosting::add_to_waitlist();
  |                                     ^^^^^^^^^^^^^^^ private function
  |
note: the function `add_to_waitlist` is defined here
 --> src/lib.rs:3:9
  |
3 |         fn add_to_waitlist() {}
  |         ^^^^^^^^^^^^^^^^^^^^

error[E0603]: function `add_to_waitlist` is private
  --> src/lib.rs:12:30
   |
12 |     front_of_house::hosting::add_to_waitlist();
   |                              ^^^^^^^^^^^^^^^ private function
   |
note: the function `add_to_waitlist` is defined here
  --> src/lib.rs:3:9
   |
3  |         fn add_to_waitlist() {}
   |         ^^^^^^^^^^^^^^^^^^^^

For more information about this error, try `rustc --explain E0603`.
error: could not compile `restaurant` due to 2 previous errors
```

We still come up with an error, so what happened? Adding the `pub` keyword in front of `mod hosting` makes the module public. With this change, if we can access `front_of_house`, we can access `hosting`. But the *contents* of `hosting` are still private; making the module public doesn’t make its contents public.

Let’s also make the `add_to_waitlist` function public by adding the `pub` keyword before its definition:

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}
```

Now the code finally compiles.

### Starting Relative Paths with `super`

- We can also construct relative paths that begin in the parent module by using `super` at the start of the path. This is like starting a filesystem path with the `..` syntax.

Consider the following code, that models the situation in which a chef fixes an incorrect order and personally brings it out to the customer. The function `fix_incorrect_order` calls the function `serve_order` by specifying the path to `serve_order` starting with `super`:

```rust
fn serve_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::serve_order();
    }

    fn cook_order() {}
}
```

The `fix_incorrect_order` function is in the `back_of_house` module, so we can use `super` to go to the parent module of `back_of_house`, which in this case is `crate`, the root. From there, we look for `serve_order` and find it.

### Making structs and enums public

- If we use `pub` before a struct definition, we make the struct public, but the struct’s fields will still be private.
- We can make each field public or not on a case-by-case basis. Here we’ve defined a public `back_of_house::Breakfast` struct with a public `toast` field but a private `seasonal_fruit` field. This models the case in a restaurant where the customer can pick the type of bread that comes with a meal, but the chef decides which fruit accompanies the meal based on what’s in season and in stock.

```rust
mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

pub fn eat_at_restaurant() {
    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");
}
```

Because the `toast` field in the `back_of_house::Breakfast` struct is public, in `eat_at_restaurant` we can write and read to the `toast` field using dot notation. Notice that we can’t use the `seasonal_fruit` field in `eat_at_restaurant` because `seasonal_fruit` is private.

- In contrast, if we make an enum public, all of its variants are then public. We only need the `pub` before the `enum` keyword

```rust
mod back_of_house {
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant() {
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}
```

Because we made the `Appetizer` enum public, we can use the `Soup` and `Salad` variants in `eat_at_restaurant`.

- Enums aren’t very useful unless their variants are public; it would be annoying to have to annotate all enum variants with `pub` in every case, so the default for enum variants is to be public.
- Structs are often useful without their fields being public, so struct fields follow the general rule of everything being private by default unless annotated with `pub`.

## Bringing Paths into Scope with the `use` keyword

- When writting the absolute and relative paths it might seem like the paths we’ve written to call functions so far are inconveniently long and repetitive.
- Fortunately, there’s a way to simplify this process. We can bring a path into a scope once and then call the items in that path as if they’re local items with the `use` keyword.

Here is a quick example of how we can call `add_to_waitlist` in a faster pace:

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}
// Absolute path example
use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}
```

- Adding `use` and a path in a scope is similar to creating a symbolic link in the filesystem. By adding `use crate::front_of_house::hosting` in the crate root, `hosting` is now a valid name in that scope, just as though the `hosting` module had been defined in the crate root. Paths brought into scope with `use` also check privacy, like any other paths.

Let’s demonstrate how we can specify a relative path to get the same behaviour:

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}
// Relative path example
use self::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}
```

### Create Idiomatic `use` Paths

- Now you might ask yourself, why we specified `use crate::front_of_house::hosting` and then called `hosting::add_to_waitlist` in `eat_at_restaurant` rather than specifying the `use` path all the way to `add_to_waitlist` function to achieve the same result.

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}
// This code is unclear as to where add_to_waitlist is defined
use crate::front_of_house::hosting::add_to_waitlist;

pub fn eat_at_restaurant() {
    add_to_waitlist();
    add_to_waitlist();
    add_to_waitlist();
}
```

The answer is the following one, although both options accomplish the same task, specifying use until the `hosting` part is the idiomatic way to bring a function into scope with `use`. Bringing the function’s parent module into scope with `use` means we have to specify the parent module when calling the function. Using `use crate::front_of_house::hosting::add_to_waitlist;` is unclear because we don’t know where exactly add_to_waitlist is defined.

When bringing `use`, it’s idiomatic to specify the full path, in the following code we show the idiomatic way to bring the standard library’s `HashMap` struct into the scope of a binary crate:

```rust
use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}
```

Let’s demonstrate why being idiomatic is the way to go in Rust by tackling the following program:

```rust
// The parent modules distinguishes the two Result types
use std::fmt;
use std::io;

fn function1() -> fmt::Result {
    // --snip--
    Ok(())
}

fn function2() -> io::Result<()> {
    // --snip--
    Ok(())
}
```

If instead we specified `use std::fmt::Result` and `use std::io::Result`, we’d have two `Result` types in the same scope and Rust wouldn’t know which one we meant when we used `Result`.

### Providing New Names with the `as` Keyword

- There’s another solution to the problem of bringing two types of the same name into the same scope with `use`: after the path, we can specify `as` and a new local name, or alias, for the type.

Here is an example of how we can rename the two `Result` types using `as`:

```rust
use std::fmt::Result;
// We can now refer to io::Result as IoResult
// With the help of as, we won't have any conflict with the Result type from std::fmt
use std::io::Result as IoResult;

fn function1() -> Result {
    // --snip--
    Ok(())
}

fn function2() -> IoResult<()> {
    // --snip--
    Ok(())
}
```

### Re-exporting Names with `pub` use

- When we bring a name into scope with the `use` keyword, the name available in the new scope is private.
- We can combine `use` and `pub` to make our code defined in that code’s scope. This technique is called *re-exporting*
 because we’re bringing an item into scope but also making that item available for others to bring into their scope.

Here is an example:

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}
/* If we hadn't specified pub use, only
eat_at_restaurant could have take advantage of calling hosting::add_to_waitlist*/
pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}
```

### Using External Packages

- Back in [Programming a Guessing Game](https://www.notion.so/Programming-a-Guessing-Game-85da9ab599414cea9ff27b3fea4ad20b) we used an external package called `rand` to get random numbers. To use this external package, we added this line to *Cargo.toml:*

```bash
rand = "0.8.3"
```

By adding this to *Cargo.toml* we tell Cargo to download the `rand` package and any dependencies from [crates.io](https://crates.io/)

- Then, to bring `rand` definitions into the scope of our package, we added `use` line starting with the name of the crate and the specific item we want to bring into scope:

```rust
use rand::Rng;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..101);
}
```

- Any packages available inside of [crates.io](https://crates.io/) are available for download, and pulling them into our workingspace involves these steps: listing them in your package’s *Cargo.toml* file and using `use` to bring items from their creates into scope.

**Note:** The standard library `std` is also a crate that is external to our package. Because the standard library is shipped with the Rust language, we don’t need to make any changes to our Cargo.toml file. For example with HashMap we would use:

```rust
use std::collections::HashMap;
```

### Using Nested Paths to Clean Up Large `use` Lists

- Here is another feature that Rust use that is also available in [Golang](https://go.dev/)

```go
// We can import multiple packages with a pair of parentheses.
import (
	"fmt"
	// We can assign an alias to a package by including the alias before the package name.
	t "time"
	// We can now call functions from time by using t instead of writting time.
)

func main() {
	fmt.Println("Hello, 世界")
	fmt.Println(t.Now())
}
```

- If we’re using multiple items defined in the same crate or same module, listing each item on its own line can take up a lot of vertical space in our files. For example, bringing the two use statements takes space:

```rust
// --snip--
use std::cmp::Ordering;
use std::io;
// --snip--
```

- Instead, we can use nested paths to bring the same items into scope in one line. We do this by specifying the common part of the path, followed by two colons, and then curly brackets

```rust
// --snip--
use std::{cmp::Ordering, io};
// --snip--
```

- In bigger programs, bringing many items into scope from the same crate or module using nested paths can reduce the number of separate use statements needed by a lot!

For example we can shorten the amount of code for importing the following:

```rust
use std::io;
use std::io::Write;
// We can shorten this to:
use std::io::{self, Write};
```

### The Glob Operator

- If we want to bring all public items defined in a path into scope, we can specify that path followed by the `*` glob operator:

```rust
use std::collections::*;
```

This `use` statement brings all public items defined in `std::collections` into the current scope. With great power comes great responsability, be careful when using the glob operator! Glob can make it harder to tell what names are in scope and where a name used in your program was defined.

## Separating Modules into Different Files

- So far, all the examples in this chapter defined multiple modules in one file. When modules get large, you might want to move their definitions to a separate file to make the code easier to navigate.

Let’s start by illustrating this idea by moving the `front_of_house` module to its own file src/front_of_house.rs. In this case, the crate root is src/lib.rs, but this procedure also works with binary crates whose crate root file is src/main.rs

**src/main.rs**

```rust
mod front_of_house;

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}
```

And src/front_of_house.rs gets the definition from the body of the front_of_house module, as shown:

**src/front_of_house.rs**

```rust
pub mod hosting {
    pub fn add_to_waitlist() {}
}
```

To continue with our example and extract the `hosting` module to its own file as well, we change *src/front_of_house.rs* to contain only the declaration of the `hosting` module:

**src/front_of_house.rs**

```rust
pub mod hosting;
```

Then we create a *src/front_of_house* directory and a file *src/front_of_house/hosting.rs* to contain the definitions made in the `hosting` module:

**src/front_of_house/hosting.rs**

```rust
#![allow(unused)]
fn main() {
pub fn add_to_waitlist() {}
}
```

The module tree remains the same, and the function calls in `eat_at_restaurant` will work without any modification, even though the definitions live in different files.

- The `mod` keyword declares modules, and Rust looks in a file with the same name as the module for the code that goes into that module.

### Summary

- Rust lets you split a package into multiple crates and a crate into modules so you can refer to items defined in one module from another module.
- You can do this by specifying absolute or relative paths. These paths can be brought into scope with a `use` statement so you can use a shorter path for multiple uses of the item in that scope.
- Module code is private by default, but you can make definitions public by adding the `pub` keyword.