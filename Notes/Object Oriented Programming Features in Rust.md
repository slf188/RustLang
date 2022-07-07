# Object Oriented Programming Features in Rust

- OOP is a way of modeling programs.
- Objects came from Simula in 1960.
- Those objects influenced Alan Kay’s programming architecture in which objects pass messages to each other.
- Alan Kay coined the term oop in 1967 to describe his architecture.
- Here we will show how to implement oop design patterns in Rust.

## Characteristics of Object oriented languages

- Arguably, OOP languages share certain common characteristics, namely objects, encapsulation, and inheritance.

### Objects contain data and behaviour

- The book *Design Patterns: Elements of Reusable Object-Oriented Software,* colloquially referred to as *The Gang of Four* book defines OOP in the following way:

<aside>
💡 Object-oriented programs are made up of objects. An *object* packages both data and the procedures that operate on that data. The procedures are typically called *methods* or *operations*.

</aside>

- Using this definition, Rust is object oriented: structs and enums have data, and `impl` blocks provide methods on structs and enums.
- Even though structs and enums with methods aren’t *called* objects, they provide the same functionality, according to the Gang of Four’s definition of objects.

### Encapsulation that hides implementation details

- Another aspect commonly associated with OOP is the idea of *encapsulation*, which means that the implementation details of an object aren’t accessible to code using that object.
- We can control encapsulation by using the pub keyword to decide which modules, types, functions and methods in our code should be public, and by default everything is private.

```rust
pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}
// the structs is marked as pub so other code can use it
// but the fields remain private
```

Here are a few methods implemented within the struct:

```rust
impl AveragedCollection {
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            }
            None => None,
        }
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}
```

- The public methods `add`, `remove`, and `average` are the only ways to access or modify data in an instance of `AveragedCollection`.
- If encapsulation is a required aspect for a language to be considered object oriented, then Rust meets that requirement. The option to use `pub` or not for different parts of code enables encapsulation of implementation details.

### Inheritance as a type system and as code sharing

- *Inheritance* is a mechanism whereby an object can inherit from another object’s definition, thus gaining the parent object’s data and behavior without you having to define them again.
- If a language must have inheritance to be an object-oriented language, then Rust is not one.
- There is no way to define a struct that is able to inherit the parents structure, however, if we need something similar we can use other solutions in Rust.
- We may choose inheritance for two main reasons, one is for reusing code.
- The other reason to use inheritance relates to the type system: to enable a child type to be used in the same places as the parent type. This is also called *polymorphism*, which means that you can substitute multiple objects for each other at runtime if they share certain characteristics.

<aside>
🗺️ **Polymorphism:** Means code that can work with data of multiple types. For inheritance those types are generally subclasses. Rust can implement such behaviour by using generics to abstract over different possible types sometimes called *bounded parametric polymorphism.*

</aside>

- Inheritance has recently fallen out of favor as a programming design solution in many programming languages because it’s often at risk of sharing more code than necessary.
- It also introduces the possibility of calling methods on subclasses that don’t make sense or that cause errors because the methods don’t apply to the subclass.
- For these reasons, Rust takes a different approach, using trait objects instead of inheritance.

## Using trait objects that allow for values of different types

- Normally a vector is only able to hold elements of the same data type, previously we defined a way that allowed a vector to hold different types of elements within a single vector.
- Now it’s time to create a gui library that is going to hold different types and if we think of how we could achieve this using inheritance the process would be something like: define a class named `Component` that has a method named `draw` on it. The other classes, such as `Button`, `Image`, and `SelectBox`, would inherit from `Component` and thus inherit the `draw` method. They could each override the `draw` method to define their custom behavior
- Rust doesn’t allow the use of inheritance so here is a way to achieve a similar behaviour

### Defining a trait for common behaviour

- To implement the behavior we want `gui` to have, we’ll define a trait named `Draw` that will have one method named `draw`.
- Then we need to define a vector that takes a trait object.
- We’ve mentioned that in Rust, we refrain from calling structs and enums “objects” to distinguish them from other languages’ objects.
- Trait objects aren’t as generally useful as objects in other languages: their specific purpose is to allow abstraction across common behavior.

Here is the way to define a trait:

```rust
pub trait Draw {
    fn draw(&self);
}
```

Here is the way we would create a vector that takes a trait object:

```rust
pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}
```

Within the Screen struct we will create a method run that will call the draw method on each of its components:

```rust
impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}
```

If we would’ve wished to define the Screen struct using a generic type and trait bounds it would’ve been:

```rust
pub struct Screen<T: Draw> {
    pub components: Vec<T>,
}

impl<T> Screen<T>
where
    T: Draw,
{
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}
```

### Implementing the trait

- Now we’ll add some types that implement the `Draw` trait.
- We will provide the Button type

```rust
pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        // code to actually draw a button
    }
}
```

- The `width`, `height`, and `label` fields on `Button` will differ from the fields on other components.

If someone using our library decides to implement a `SelectBox` struct that has `width`, `height`, and `options` fields, they implement the `Draw` trait on the `SelectBox` type as well, as shown here:

```rust
use gui::Draw;

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        // code to actually draw a select box
    }
}
```

- Our library’s user can now write their `main` function to create a `Screen` instance.

```rust
use gui::{Button, Screen};

fn main() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };

    screen.run();
}
```

- Rust won’t compile our code if the values don’t implement the traits that the trait objects need.

For example, what happens if we try to create a Screen with a string as a component:

```rust
use gui::Screen;

fn main() {
    let screen = Screen {
        components: vec![Box::new(String::from("Hi"))],
    };

    screen.run();
}
// We get this error because string doesn't implement the Draw trait
$ cargo run
   Compiling gui v0.1.0 (file:///projects/gui)
error[E0277]: the trait bound `String: Draw` is not satisfied
 --> src/main.rs:5:26
  |
5 |         components: vec![Box::new(String::from("Hi"))],
  |                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `Draw` is not implemented for `String`
  |
  = note: required for the cast to the object type `dyn Draw`

For more information about this error, try `rustc --explain E0277`.
error: could not compile `gui` due to previous error
```

### Trait objects perform dynamic dispatch

- Dynamic dispatch is when the compiler can’t tell at compile time which method you’re calling.
- In dynamic dispatch cases, the compiler emits code that at runtime will figure out which method to call.

## Implementing an OOP design pattern

- The state pattern is an oop design pattern
- The state pattern has an internal state represented by a set of state objects and the value’s behaviour changes based on the internal state.
- The state objects share functionality in Rust of course we use structs and traits rather than objects and inheritance
- Using the state pattern means when the business requirements of the program changes, we won’t need to change the code of the value holding the state, instead, we will only need to update the code inside one of the state objects to change its rules or add more state objects.

To see how we can implement oop functionality we will take the following example and try to replicate it:

- A blog post workflow
    1. A blog starts as an empty draft
    2. When the draft is done, a review of the post is requested
    3. Once the post is approved, it gets published
    4. Only the published posts return content to print, unapproved posts can’t accidentally be published

### Defining Post and creating a new instance in the draft state

```rust
pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }
}
// behaviour shared by different post states
trait State {}

struct Draft {}

impl State for Draft {}
```

### Storing the text of the post content

This means we can implement a method later that will control how the `content` field’s data is read. The `add_text` method is pretty straightforward so the implementation would be:

```rust
impl Post {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }
}
```

### Ensuring the content of a draft post is empty

We aim to see that the content method will return an empty string slice 

```rust
impl Post {
    // --snip--
    pub fn content(&self) -> &str {
        ""
    }
}
```

### Requesting a review of the post changes its state

Next, we need to add functionality to request a review of a post, which should change its state from `Draft` to `PendingReview`.

```rust
impl Post {
    // --snip--
    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }
}

trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
}

struct Draft {}

impl State for Draft {
		// consume the current state and return a new state
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }
}

struct PendingReview {}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
}
```

### Adding the approve method that changes the behaviour of content

The `approve` method will be similar to the `request_review` method: it will set `state` to the value that the current state says it should have when that state is approved:

```rust
impl Post {
    // --snip--
    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }
}

trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
}

struct Draft {}

impl State for Draft {
    // --snip--
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

struct PendingReview {}

impl State for PendingReview {
    // --snip--
    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }
}

struct Published {}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
}
```

```rust
trait State {
    // --snip--
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        ""
    }
}

// --snip--
struct Published {}

impl State for Published {
    // --snip--
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}
```

### Encoding states and behaviour as types

Rather than encapsulating the states and transitions completely so outside code has no knowledge of them, we’ll encode the states into different types.

```rust
fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());
}
```

The definition of the Post and DraftPost struct are the following ones:

```rust
pub struct Post {
    content: String,
}

pub struct DraftPost {
    content: String,
}

impl Post {
    pub fn new() -> DraftPost {
        DraftPost {
            content: String::new(),
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}

impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }
}
```

### Implementing transitions as transformations into different types

So to get a published post, we want to enforce a rule that a draft post has to be reviewed before it can be published, the constraints would then be:

```rust
impl DraftPost {
    // --snip--
    pub fn request_review(self) -> PendingReviewPost {
        PendingReviewPost {
            content: self.content,
        }
    }
}

pub struct PendingReviewPost {
    content: String,
}

impl PendingReviewPost {
    pub fn approve(self) -> Post {
        Post {
            content: self.content,
        }
    }
}
```

To call these then it would be:

```rust
use blog::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");

    let post = post.request_review();

    let post = post.approve();

    assert_eq!("I ate a salad for lunch today", post.content());
}
```

### Summary

- Now we know we can use trait objects to get some object-oriented features in Rust.
- Dynamic dispatch can give your code some flexibility in exchange for a bit of runtime performance.
- Rust also has other features, like ownership, that object-oriented languages don’t have. An object-oriented pattern won’t always be the best way to take advantage of Rust’s strengths, but is an available option.