# Fearless Concurrency

Here is another great concept in programming we should tackle:

**Concurrent Programming:** where different parts of a program execute independently.

**Parallel Programming:** where different parts of a program execute at the same time.

- Historically, programming in these contexts has been difficult and error prone: Rust aims to change that.
- Fearless concurrency allows you to write code that is free of subtle bugs and is easy to refactor without introducing new bugs.

## Using threads to run multiple pieces of code at the same time

Let’s define what a thread is:

- In most os, an executed program’s code is run in a *process*, and the operating system will manage multiple processes at once.
- Within a program you can also have independent parts that run simultaneously.
- The features that run these independent parts are threads.
- So having multiple threads means the os could respond to more than one request at the same time.
- So in simple terms, splitting the computation in your program into multiple threads to run multiple tasks at the same time can improve performance, but it also adds complexity.
- And because threads run simultaneously, there is no guarantee about the order in which the parts of your code on different threads will run. This could lead to problems such as
    - Race conditions
    - Deadlocks
    - Bugs

### Creating a new thread with spawn

- To create a new thread, we call the `thread::spawn` function and pass it a closure:

```rust
use std::thread;
use std::time::Duration;

fn main() {
    thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
}

$ cargo run
hi number 1 from the main thread!
hi number 1 from the spawned thread!
hi number 2 from the main thread!
hi number 2 from the spawned thread!
hi number 3 from the main thread!
hi number 3 from the spawned thread!
hi number 4 from the main thread!
hi number 4 from the spawned thread!
hi number 5 from the spawned thread!
```

Here is a clear example of how we can use threads to run a piece of code simultaneously! And what we see here is that the whole program has finished once the main thread ends.

### Waiting for all threads to finish using join handles

- Because there is no guarantee on the order in which threads run, we also can’t guarantee that the spawned thread will get to run at all!
- We can fix the problem of the spawned thread not running or ending prematurely by saving the return value of `thread::spawn` in a variable.

```rust
use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}

$ cargo run
hi number 1 from the main thread!
hi number 2 from the main thread!
hi number 1 from the spawned thread!
hi number 3 from the main thread!
hi number 2 from the spawned thread!
hi number 4 from the main thread!
hi number 3 from the spawned thread!
hi number 4 from the spawned thread!
hi number 5 from the spawned thread!
hi number 6 from the spawned thread!
hi number 7 from the spawned thread!
hi number 8 from the spawned thread!
hi number 9 from the spawned thread!
```

- Calling `join` on the handle blocks the thread currently running until the thread represented by the handle terminates.
- Let’s see what happens if we move the handle.join before the for loop in main

```rust
use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    handle.join().unwrap();

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
}
// The main thread will wait for the spawned thread to finish and then run its for 
// loop, so the output won’t be interleaved anymore
$ cargo run
hi number 1 from the spawned thread!
hi number 2 from the spawned thread!
hi number 3 from the spawned thread!
hi number 4 from the spawned thread!
hi number 5 from the spawned thread!
hi number 6 from the spawned thread!
hi number 7 from the spawned thread!
hi number 8 from the spawned thread!
hi number 9 from the spawned thread!
hi number 1 from the main thread!
hi number 2 from the main thread!
hi number 3 from the main thread!
hi number 4 from the main thread!
```

- Small details as where join is called can affect whether our threads run at the same time.

### Using move closures with threads

- We'll often use the `move` keyword with closures passed to `thread::spawn` because the closure will then take ownership of the values it uses from the environment, thus transferring ownership of those values from one thread to another.
- To use data from the main thread in the spawned thread, the spawned thread’s closure must capture the values it needs.

```rust
use std::thread;

fn main() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
}

$ Here's a vector: [1, 2, 3]
```

- By adding the `move` keyword before the closure, we force the closure to take ownership of the values it’s using rather than allowing Rust to infer that it should borrow the values.

## Using m*essage-passing* concurrency, where channels send messages between threads

- A popular approach to ensure safe concurrency is by using message passing, where threads communicate with each other by sending messages containing data.
- Here’s the idea in a slogan from [the Go language documentation](https://golang.org/doc/effective_go.html#concurrency): **“Do not communicate by sharing memory; instead, share memory by communicating.”**
- To accomplish such feat, Rust provides an implementation of channels. A channel is a general concept by which data is sent from one thread to another.
- A channel has two halves: a transmitter and a receiver. Think of the two neurons communicating with each other.

Here we will make an showcase example of how one thread generates a value, sends them down a channel and another thread will receive the values and print them out.

```rust
use std::sync::mpsc;

fn main() {
		// mpsc stands for multiple producer, single consumer
    let (tx, rx) = mpsc::channel();
		// the function returns a tuple, 
		// the first element is the sending-end transmitter and...
		// the second element is the receiving-end receiver
		// tx means transmitter and rx means receiver
}
```

Here is another real life example, here we will set up the transmitter

```rust
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });
}
```

Here we will setup the receiver:

```rust
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });
		// the receiver has two useful functions
		// the recv function is short for receive
    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}

$ cargo run
Got: hi
```

### Channels and ownership transference

- The ownership rules play a vital role in ensuring us write safe, concurrent code.

### Sending multiple values and seeing the receiver waiting

```rust
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}

$ cargo run
Got: hi
Got: from
Got: the
Got: thread
```

- This time, the spawned thread has a vector of strings that we want to send to the main thread.
- We iterate over them, sending each individually and pause between each by calling the `thread::sleep` function with a duration value of 1 second.

### Creating multiple producers by cloning the transmitter

Let’s put mpsc to use and expand the code to create multiple threads that all send values to the same receiver.

```rust
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}

$ cargo run
Got: hi
Got: more
Got: from
Got: messages
Got: for
Got: the
Got: thread
Got: you
```

## *Shared-state* concurrency

- Message passing is not the only way to handle concurrency, another method would be to allow multiple threads to access the same shared data.
- So that being said let us review mutexes, a concurrency primitive for shared memory.

### Using mutexes to allow access data from one thread at a time

- Mutex is an abbreviation for mutual exclusion.
- To access data in a mutex, a thread must first signal that it wants access by asking to acquire the mutex’s lock.
- The lock is a data structure that is part of the mutex that keeps track of who currently has exclusive right to the data
- Therefore mutex is known as guardian of data that it holds via the locking system.
- Mutex are difficult to use because:
    - Acquire the lock before using the data
    - Unlock the data so the other threads can acquire the lock

### The API of Mutex<T>

Let’s start by demonstrating how to use mutex in a single-threaded context:

```rust
use std::sync::Mutex;

fn main() {
		// create mutex using its function to access the data
    let m = Mutex::new(5);

    {
				// we acquire the lock with the lock method and subsequently unwrap it
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {:?}", m);
}

$ cargo run
m = Mutex { data: 6, poisoned: false, .. }
```

### Sharing a Mutex<T> between multiple threads

Let’s try to share a value between multiple threads using Mutex<T>, to do so we need to integrate an atomic reference counting with Arc<T>, it is a type like `Rc<T>` that is safe to use in concurrent situations. A stands for atomatically reference counted type, they work like primitive types but are safe to share across threads.

```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}

$ cargo run
Result: 10
```

### Similarities between RefCell<T>/Rc<T> and Mutex<T>/Arc<T>

- You might have noticed that `counter` is immutable but we could get a mutable reference to the value inside it; this means `Mutex<T>` provides interior mutability, as the `Cell` family does.
- Another detail to note is that Rust can’t protect you from all kinds of logic errors when you use `Mutex<T>`.
- Mutex comes with risk of creating deadlocks. These occur when an operation needs to lock two resources and two threads have each acquired one of the locks, causing them to wait for each other forever.

## Extensible concurrency with the `Sync` and `Send` traits

- Interestingly, the Rust language has *very* few concurrency features. The concurrency features we’ve talked about are most of the features that are being offered in the standard library.
- However, two concurrency concepts are embedded in the language: the `std::marker` traits `Sync` and `Send`.

### Allowing transference of ownership between threads with send

- The `Send` marker trait indicates that ownership of values of the type implementing `Send` can be transferred between threads.
- Almost all primitive types are `Send`, aside from raw pointers.

### Allowing access from multiple threads with sync

- The `Sync` marker trait indicates that it is safe for the type implementing `Sync` to be referenced from multiple threads.
- In other words, any type `T` is `Sync` if `&T` (an immutable reference to `T`) is `Send`, meaning the reference can be sent safely to another thread.

### Implementing send and sync manually is unsafe

- Because types that are made up of `Send` and `Sync` traits are automatically also `Send` and `Sync`, we don’t have to implement those traits manually.
- As marker traits, they don’t even have any methods to implement. They’re just useful for enforcing invariants related to concurrency.
- Manually implementing these traits involves implementing unsafe Rust code.

## Summary

- As mentioned earlier, because very little of how Rust handles concurrency is part of the language, many concurrency solutions are implemented as crates.
- The Rust standard library provides channels for message passing and smart pointer types, such as `Mutex<T>`
 and `Arc<T>`, that are safe to use in concurrent contexts.
- Concurrent programming is no longer a concept to be afraid of: go forth and make your programs concurrent, fearlessly!