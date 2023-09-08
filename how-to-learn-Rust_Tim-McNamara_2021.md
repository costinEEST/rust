> [Video talk](https://youtu.be/sDtQaO5_SOw)

- Tim's beliefs:

  - Rust is centered around traits and safe access to data
  - When starting to learn Rust, focus on [structs](https://doc.rust-lang.org/rust-by-example/custom_types/structs.html), [vectors](https://doc.rust-lang.org/rust-by-example/std/vec.html), [iteration](https://doc.rust-lang.org/rust-by-example/flow_control.html), [Result](https://doc.rust-lang.org/rust-by-example/error/result.html) and [Option](https://doc.rust-lang.org/rust-by-example/error/option_unwrap.html).

- Where should I start?

```rust
fn main() {
    println!("Hallo, Linz");
}
```

- How do I use a variable?

```rust
fn main() {
    let name = "Linz";

    println!("Hallo, {}!", name);
}
```

- How do I create my own type?

```rust
struct Greeting {
    name: String
}

fn main() {
    let greeting = Greeting {name: "Linz".to_string()};

    println!("Hallo, {}!", greeting.name);
}
```

- What is a constructor?

```rust
struct Greeting {
    name: String,
}

impl Greeting {
    fn new(name: &str) -> Self {
        Greeting {
            name: name.to_string(),
        }
    }
}

fn main() {
    let greeting = Greeting::new("Linz");

    println!("Hallo, {}!", greeting.name);
}
```

- How do I accept a `&str` or a `String` ?

```rust
struct Greeting {
    name: String,
}

impl Greeting {
    fn new<T: AsRef<str>>(name: T) -> Self {
        Greeting {
            name: name.as_ref().to_string(),
        }
    }
}

fn main() {
    let greeting = Greeting::new("Linz");

    println!("Hallo, {}!", greeting.name);
}
```

- How can I ask a _Greeting_ to print itself ?

```rust
use std::fmt;

struct Greeting {
    name: String,
}

impl Greeting {
    fn new<T: AsRef<str>>(name: T) -> Self {
        Greeting {
            name: name.as_ref().to_string(),
        }
    }
}

impl fmt::Display for Greeting {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Hallo, {}!", self.name)
    }
}

fn main() {
    let greeting = Greeting::new("Linz");

    println!("{}", greeting);
}
```

- Rust can feel magical, it is common to be confused

```rust
// std::mem::drop

// The underscore annotation signals that the argument might not be used
pub fn drop<T>(_x: T) {}
```

```rust
use std::collections::HashMap;

let mut letters = HashMap::new();

for ch in "some text".chars() {
    // 'counter' is a mutable reference for what it is inside of the HashMap
    let counter = letters.entry(ch).or_insert(0);

    // dereference it
    *counter += 1;
}
```

```rust
// std::convert::Intro;

impl<T, U> Into<U> for T
where U: From<T>, {
    fn into(self) -> U {
        U::from(self)
    }
}
```

- Give yourself permission to use `.clone()` and `.to_string()`. Memory allocations only require dozens of nanoseconds.
- There is more than one way to achieve a particular outcome:

```rust
fn main() {
    let needle = 0o204;
    let haystack = vec![1, 2, 5, 132, 429, 1430];

    for item in &haystack {
        if *item == needle {
            println!("{}", item);
            break;
        }
    }

    // or
    if haystack.contains(&needle) {
        println!("{}", needle);
    }
}
```

```rust
fn main() {
    let penguin_data = "\
    common name, length (cm)
    Little penguin, 33
    YellowEyed penguin, 65
    Fiord land penguin, 60
    Invalid, data
    ";

    let records = penguin_data.lines();

    for record in records {
        let fields: Vec<_> = record
            .split(',')
            .map(|field| field.trim())
            .collect();

        let name = fields[0];
        let height: Result<f32, _> = fields[1].parse();
        if height.is_err() {
            continue;
        }

        println!("{} has {} cm", name, height.unwrap());
    }
}
```
