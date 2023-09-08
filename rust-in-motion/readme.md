# CLI

- [`rustup`](https://rustup.rs) is a toolchain multiplexer. It installs and manages many Rust toolchains and presents them all through a single set of tools installed to `~/.cargo/bin`. The `rustc` and `cargo` executables installed in `~/.cargo/bin` are proxies that delegate to the real toolchain.
- Package manager version: `cargo --version`
- Compiler version: `rustc --version`
- Create a new binary crate: `cargo new --bin project_name`
- Create a new library crate: `cargo new --lib project_name`
- Compile the code: `cargo build`
- Open the official online documentation: `rustup doc`

# Dump

- https://github.com/integer32llc/rust-in-motion-videos
- [Organizations running Rust in production](https://prev.rust-lang.org/en-US/friends.html)
- [`std::fmt`](https://doc.rust-lang.org/stable/std/fmt/index.html)
- [Concise answers to common questions about floating-point numbers](https://github.com/brazzy/floating-point-gui.de)
- https://www.manning.com/books/rust-in-action

# Notes

- Scalar type: boolean, integer, floating point, character.
- Compound type: array, tuple, slice.
- Arrays properties:
  - every element must have the same type.
  - have a fixed length.
  - are stored in the `stack` (data stored in it can be accessed swiftly).
- Tuples properties:
  - tuples, like arrays have a fixed length.
  - elements can be of same/different scalar data types.
  - stored on the `stack` (faster access).
- Idiomatic ways to return a value from a function:
  - end the function with an expression resulting in the value to be returned and don’t end that line with a semicolon.
  - use the return keyword and a semicolon when returning early.
- An `enum` is for when you want to describe a choice between a set of values, such as choosing banana, apple, or orange from a set of fruits.
- Aspects of enums:
  - the variants are defined at compile time, so new variants cannot be added at runtime.
  - a value can only be one variant of an enum at a time.
  - variants can optionally be defined to hold data.
- A `struct` is for when you want to have the same attributes for all of the values of that type,
  such as multiple recipes that all have a list of ingredients and a list of directions:
  - `tuple structs` are structs that have a name for the whole type but don't name their fields. They behave similarly to tuples. You access their fields with a dot and an index, starting with zero.
  - `unit structs` are field-less, used for generics.
  - classic [C structs](<https://en.wikipedia.org/wiki/Struct_(C_programming_language)>)
- Methods are defined either in the context of an `enum` or a `struct`. Methods are defined within a block that starts with the `impl` keyword, which is short for implementation. Then we specify what we're _implementing_ methods on. A big difference between functions and methods is that the first parameter of a method is always a form of `self`, which refers to the type that we're defining the method on. The rest of the parameters of the method are the same as the rest of the parameters of the function we had.
- Associated functions are defined within an `impl` block on a `struct` or an `enum`, the same as methods. They're still behavior that's related to the type, so we want to organize that behavior so that it's associated with the type. Unlike methods, they don't take `self` as a parameter. Associated functions are commonly used to create instances of types, so they don't have a `self` instance to operate on.
- Rust's system of `ownership` differs from some other languages, such as manual memory management in C and garbage collection in Ruby:
  - in C, your code has to explicitly perform both allocation and deallocation. When you want space to store some data, you call `malloc` and specify the amount of memory you want. When you're done with that space, you call `free` to return the memory to the operating system.
  - the advantage of this is that you have complete control over when your program allocates memory and how much it uses. This control lets you write programs that run quickly and take up the least amount of space.
  - however, having complete control is also a disadvantage. C, by itself, doesn't prevent you from messing this up. If one part of your program calls `free` and another part tries to use that memory again, that's called a "use after free" problem.
  - if you forget to call `free` once you're done with the memory, eventually you'll run out of space. This is a memory leak.
  - if two parts of your program try to clean up the same data, that's known as a "double free" problem and can cause memory corruption. There are tools to help diagnose and debug these memory problems, but the C language doesn't help us out very much.
  - check [Chapter 12](https://livebook.manning.com/book/modern-c/chapter-12) and [Chapter 13](https://livebook.manning.com/book/modern-c/chapter-13) in _Modern C_, to learn more about memory management in C.
  - Ruby, on the other hand, takes care of managing memory for you, and has a component called the garbage collector that runs alongside your program cleaning up memory that you're done with.
  - the advantage of a garbage collector is that, while you're writing code, you don't really have to think about managing memory at all. The garbage collector keeps track of which memory is still in use, and cleans up any data that your program is done with.
  - you also can't have any of the problems we mentioned with C, because Ruby is taking care of all that for you. The downside of using a garbage collector is that you lose control and performance.
  - when it runs, the garbage collector takes up computing resources that your program could be using. Your program will also likely use more memory than it strictly needs because the garbage collector only cleans up memory when it's sure you're done with it.
  - Rust is trying to get the best of both worlds here. Ownership gives you control over memory allocation and the associated performance, but by cleaning up data automatically when the owner goes out of scope, we can't mess up the memory access and we won't be using memory longer than we strictly need to.
- A `String` is a type that has data that needs to be cleaned up when it goes out of scope. A `String` value tracks how much space is allocated, how much of that space is used, and the UTF-8 data. When the owner of the `String` value goes out of scope, the UTF-8 data needs to be cleaned up.
- With non-primitive types, Rust moves ownership. If we have the variable, `s`, holding a `String`, and we create a new variable, `t`, and assign `s` to `t`, this will move ownership of the `String` from `s` to `t`. Afterwards, the variable `s` is no longer available to be used.
- What if we wanted to keep ownership of some data, but needed to also give ownership of the data to another piece of code? We can do that by _cloning_. Cloning makes a deep copy of the allocated memory. Then there will be two copies of the data that each have an owner and each owner is responsible for cleaning up their data.
- [Chapter 4](https://livebook.manning.com/book/rust-in-action/chapter-4) of _Rust in Action_ provides more information about ownership in Rust.

```rust
fn pluralize(word: String) -> String {
    word + "s"
}

fn main() {
    let s = String::from("book");

    println!(
        "I have one {}, you have two {}",
        s,
        // Not idiomatic, but works. Use borrowing instead
        pluralize(s.clone())
    );
}
```

```rust
fn emphasize(single: String) -> String {
    single + "!"
}

fn main() {
    let s = String::from("hi");

    let emphasized_once = emphasize(s);
    // Not idiomatic, but works. Use borrowing instead
    let emphasized_twice = emphasize(emphasized_once.clone());

    println!("Once: {}, twice: {}", emphasized_once, emphasized_twice);
}
```

- Borrowing is a way to allow some code to use a value without moving ownership.
- The reason we want the ability to borrow is for performance. If a function doesn't need ownership of a value that has allocated memory, instead of cloning the value and giving that to the function, we can give the function a reference to the original value. The function can borrow the value and no extra allocation is needed.
- Borrowing and references may remind you of pointers in languages like C and C++. References in Rust are similar – there is one big, important difference. In safe Rust, the borrow checker ensures, at compile time, that you'll never have an invalid reference – a reference that points to nothing or to invalid memory. This prevents a lot of bugs.
- If you've ever gotten a "seg fault" in C or C++, or errors like `undefined method for nil class` in Ruby, or `undefined is not a function` in JavaScript, you've experienced a runtime bug caused by an invalid reference that's a compile time error in Rust.

```rust
fn pluralize(word: &String) -> String {
    word.to_owned() + "s"
}

fn main() {
    let s = String::from("book");

    println!(
        "I have one {}, you have two {}",
        s,
        pluralize(&s)
    );
}
```

```rust
struct Cat {
    name: String,
}

// Does not compile because cat 'gets' deallocated at the end of the 'name' function, so the returned reference wouldn’t be valid
fn name() -> &str {
    let cat = Cat {
        name: String::from("Mx. Fuzzypants"),
    };
    &cat.name
}
```

- A slice is a data type that always borrows data owned by some other data structure, such as the `String`.
- A slice consists of a pointer and a length. The pointer is a reference to the start of the data that the slice contains, and the length is the number of elements after the start that the slice contains.
- By specifying string slices as parameters rather than borrowing an owned `String`, functions can accept either borrowed strings or string literals. String literals create string slices. Their text appears in your compiled program, which is also stored in memory, just like your data. The string slice points to this literal text.
- How it is possible to call functions that take a string slice by passing an argument that is a reference to an owned String?
  - first, the standard library includes the implementation of a trait called [`Deref`](https://doc.rust-lang.org/std/ops/trait.Deref.html) on `String`, such that Rust knows how to convert a reference to a `String` into a string slice containing the whole `String`.
  - next, Rust has a feature called _deref coercion_. This means that when you call a function or method, the compiler will automatically dereference the arguments, if need be, to convert them to match the function parameter type.
- Checked at runtime rather than at compile time:
  - String slice indices are on character boundaries
  - Slice indices are within the bounds of the data structure the slice is taken from
- The compiler enforces some rules involving mutable references: when you have a reference, you can either have any number of immutable references to a value or a single mutable reference to that value.
- Combinations allowed by the borrowing rules:

  - one mutable reference
  - one immutable reference
  - many immutable references

- A socket is a system resource that's a connection to a network endpoint for sending and receiving data.
- One kind of socket is a TCP socket, which stands for Transmission Control Protocol. TCP underlies much of the internet's communication. To use a TCP socket in code, we create it and bind it to a particular port. And we should close the socket when we're done with it to avoid overloading the system's socket capacity.
- The way sockets should be managed is similar to the way memory should be managed and they both have similar problems. It's an error to try and send or receive data on a socket after it's been closed. Trying to free memory twice is an error called "double free." Trying to close a socket twice is also an error. Memory leaks that can overwhelm the system happen if you forget to free memory. Forgetting to close a socket can also overwhelm the system.
- In languages with a garbage collector, such as Ruby, the garbage collector helps to mitigate the memory problems,but managing resources like sockets is still manual. The garbage collector only helps with memory. However,Rust helps to mitigate the issues with both memory and sockets via the same mechanism – ownership.
- Sockets aren't the only type in the standard library that are managed with ownership.Let's briefly look at a few more: Mutex<T>,Rc<T>, and File.
- The `Mutex<T>` type, which is short for _mutual exclusion_, is a data type used in multithreaded contexts to ensure that only one thread is allowed to modify the value inside the mutex.
- This is done by acquiring a lock just before modifying the value. After modifying the value, you have to release the lock to allow other threads to acquire it. In Rust, when the owner of the lock goes out of scope, the lock is automatically released. This keeps us from accidentally forgetting to release a lock.
- The `Rc<T>` type, which stands for _reference counted_, is a type that allows for multiple owners of a single piece of data. It has an internal counter of how many owners exist, and when the last owner goes out of scope, then the value is cleaned up. Decrementing the reference counter when each owner goes out of scope is handled by Rust automatically.
- Files are similar to sockets in that they must be closed when we're done with them to avoid overwhelming the system. Rust also handles closing files automatically when the owner of a File goes out of scope.
- Let's look at how to customize what our own types do when their owners go out of scope by using the `Drop` trait.
- The `Drop` trait has one method, also named drop. This method takes a mutable reference to `self` and no other parameters. This is where you'd put any logic necessary to clean up the resources a type uses.
- [CWE-126: Buffer Over-read](https://cwe.mitre.org/data/definitions/126.html)
- Panic: stop the program in invalid states.
- [`panic!`](https://doc.rust-lang.org/rust-by-example/std/panic.html) macro causes the program to stop, with optional message.
- Situations in which to panic:
  - continuing is incorrect
  - no way to recover
  - must change code
  - failure not expected
  - undecided on error handling
-
