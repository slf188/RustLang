# Getting Started

### Update

To update rust run `rustup update`

### Version

To check whether rust is installed, run the following command `rustc --version`

### Local Documentation

The installation of Rust also includes a copy of the documentation locally, so you can read it offline. Run `rustup doc` to open the local documentation.

### Writting and running a Rust program

Rust files always end with the *.rs* extension. If you’re using more than one word in your filename, use an underscore to separate them. For example, use *hello_world.rs*r ather than *helloworld.rs.*

### Anatomy of a Rust program

```rust
fn main(){
    println!("Hello world");
}
```

The `main` function is special: it is always the first code that runs in every executable Rust program.

The function body is wrapped in curly brackets, `{}`. Rust requires these around all function bodies. It’s good style to place the opening curly bracket on the same line as the function declaration, adding one space in between.

If you want to stick to a standard style across Rust projects, you can use an automatic formatter tool called `rustfmt` to format your code in a particular style.

To format your code with rustfmt you can type

```bash
rustfmt main.rs
# By running this, rust will be automatically formatted!
```

Rust style is to indent with four spaces.

`println!`calls a Rust macro. If it called a function instead, it would be entered as `println`  (without the `!`). Using a `!` means that you’re calling a macro instead of a normal function, and that macros don’t always follow the same rules as functions.

Most lines of Rust code end with a semicolon.

### Compiling

Before running a Rust program, you must compile it using the Rust compiler by entering the `rustc`command and passing it the name of your source file, like this:

```rust
rustc main.rs
ls
main   main.rs
```

After compiling successfully, Rust outputs a binary executable.

From here, you run the main file, like this:

```rust
./main
» Hello world
```

## Hello cargo

Cargo is Rust’s build system and package manager. Cargo handles a lot of tasks for you, such as building your code, downloading the libraries your code depends on, and building those libraries.

Check whether Cargo is installed by entering the following into your terminal:

```rust
cargo --version
```

### Creating a project with cargo

To do so, run the following command:

```bash
cargo new hello_cargo
# New directory created, cargo has created the files inside as well
cd hello_cargo
# cargo has generated two files: Cargo.toml and a src folder with main.rs
# it also initialized a new Git repo along with our .gitignore file
```

This file is in the *[TOML](https://toml.io/)*(*Tom’s Obvious, Minimal Language*) format, which is Cargo’s configuration format.

The first line, `[package]`, is a section heading that indicates that the following statements are configuring a package.

The next three lines set the configuration information Cargo needs to compile your program: the name, the version, and the edition of Rust to use.

The last line, `[dependencies]`, is the start of a section for you to list any of your project’s dependencies.

Cargo has generated a “Hello, world!” program for you, just like the one we wrote in Listing 1-1! So far, the differences between our previous project and the project Cargo generated are that Cargo placed the code in the *src* directory, and we have a *Cargo.toml* configuration file in the top directory.

Cargo expects your source files to live inside the *src*directory. The top-level project directory is just for README files, license information, configuration files, and anything else not related to your code. Using Cargo helps you organize your projects. There’s a place for everything, and everything is in its place.

### Building and running a cargo project

From your *hello_cargo* directory, build your project by entering the following command:

```bash
cargo build
```

Running `cargo build` for the first time also causes Cargo to create a new file at the top level: *Cargo.lock*. This file keeps track of the exact versions of dependencies in your project.

We can also use `cargo run`to compile the code and then run the resulting executable all in one command:

```bash
cargo run
» Hello world
```

Cargo also provides a command called `cargo check`. This command quickly checks your code to make sure it compiles but doesn’t produce an executable:

```bash
cargo check
```

Why would you not want an executable? Often, `cargo check` is much faster than `cargo build`, because it skips the step of producing an executable. If you’re continually checking your work while writing the code, using `cargo check` will speed up the process! As such, many Rustaceans run `cargo check` periodically as they write their program to make sure it compiles. Then they run `cargo build` when they’re ready to use the executable.

### Cargo Recap

- We can build a project using `cargo build`.
- We can build and run a project in one step using `cargo run`.
- We can build a project without producing a binary to check for errors using `cargo check`.
- Instead of saving the result of the build in the same directory as our code, Cargo stores it in the *target/debug* directory.

An additional advantage of using Cargo is that the commands are the same no matter which operating system you’re working on.

### Cargo as convention

With simple projects, Cargo doesn’t provide a lot of value over just using `rustc`, but it will prove its worth as your programs become more intricate. With complex projects composed of multiple crates, it’s much easier to let Cargo coordinate the build.