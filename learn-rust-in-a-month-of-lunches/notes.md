- `cargo doc --open`

- The signed integer types are: `i8`, `i16`, `i32`, `i64`, `i128`, and `isize`.

- The unsigned integer types are: `u8`, `u16`, `u32`, `u64`, `u128`, and `usize`.

- A `u8` can hold up to `255`,
- A `u16` can hold up to `65535`,
- And a `u128` can hold up to `340282366920938463463374607431768211455`.
- A`u8` goes from `0` to `255` while an `i8` goes from `-128` to `127`
- `isize` and `usize` on a 32-bit computer is like `i32` and `u32`, and `isize` and `usize` on a 64-bit computer is like `i64` and `u64`.

- A `usize` is the best size for indexing because:

  - An index can't be negative, so it needs to be an unsigned integer with a `u`.
  - It should have a lot of space, because index numbers can get quite large, but
  - It can't be a `u64` because 32-bit computers can't use `u64`.

- All `char`s use 4 bytes of memory, since 4 bytes are enough to hold any kind of character:

  - Basic letters and symbols usually need 1 byte to make: `a b 1 2 + - = $ @`
  - Other letters like German Umlauts or accents need 2 bytes to make: `ä ö ü ß è à ñ`
  - Korean, Japanese or Chinese characters need 3 or 4 bytes: `国 안 녕`

- The `.len()` method returns the number of bytes, not the number of letters or characters.

- `.chars().count()` gives the size in characters. Calling `.chars()` first turns a string into a collection of characters and then `.count()` counts how many of them there are.

- Since 2021, you can capture variables inside of the `{}` of `println!`
- A variable's lifetime starts and ends inside a code block: `{}`
- Three ways to print:
  - `{}` - Display print. More types have Debug than Display, so if a type you want to print can't print with Display, you can try this:
  - `{:?}` - Debug print. And if there is too much information on one line, you can try this:
  - `{:#?}` - Debug print, but pretty. 'Pretty' means that each part of a type is printed on its own line to make it easier to read.
- To see the smallest and biggest numbers, you can use `MIN` and `MAX` after the name of the type
- Rust doesn't have a garbage collector, but it is smart enough to know exactly when a variable doesn't need to exist anymore and frees the memory for you.
- Ways to declare values: `let`, `const` and `static`.
- `const` and `static` need to be typed because the Rust would not infer the type for them.
- `const` is for values that don't change, and are created at compile time.
- `static` is similar to `const`, but has a fixed memory location. It might not be created at compile time.
