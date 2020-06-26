# Rust lang

<img data-src="Rust_lang.png">

> A language empowering everyone
to build reliable and efficient software.

---

## History

- **First appeared**: July 7, 2010; 9 years ago
- Rust was originally designed at Mozilla Research by **Graydon Hoare**
- [Rust earned the top spot as the “most-loved” programming language for the fifth consecutive year of Stack Overflow’s developer survey](https://www.infoworld.com/article/3546337/rust-language-tops-stack-overflow-survey.html)



Note: speaker notes FTW!

---

## Why rust?

---

## Performance

Rust is blazingly fast and memory-efficient: with no runtime or garbage collector, it can power performance-critical services, run on embedded devices, and easily integrate with other languages.

---

## Reliability

Rust’s rich type system and ownership model guarantee memory-safety and thread-safety — enabling you to eliminate many classes of bugs at compile-time.

---

## Productivity

Rust has great documentation, a friendly compiler with useful error messages, and top-notch tooling — an integrated package manager and build tool, smart multi-editor support with auto-completion and type inspections, an auto-formatter, and more.

---

## Let's get started

---

## Primitive types

| Length  | Signed | Unsigned |
| ------- | ------ | -------- |
| 8-bit   | i8     | u8       |
| 16-bit  | i16    | u16      |
| 32-bit  | i32    | u32      |
| 64-bit  | i32    | u32      |
| 128-bit | i128   | u128     |
| arch    | isize  | usize    |

---

## Compound Types

---

## The Tuple Type

```rust
fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
}
```

```rust
fn main() {
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {}", y);
}
```

---

## The Array Type

```rust
let a = [3; 5]; //equals let a = [3, 3, 3, 3, 3];
```

```rust
fn main() {
    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];
}
```

---

## Struct type

```rust
// Exmple of generic struct
struct Complex<T> {
    //Real part
    re: T,
    //Complex part
    im: T,
}
```

```rust
// Rust impl block
impl<T> Complex<T> {
    fn new(re: T, im: T) -> Self { //Static method
        Complex { re, im }
    }
}
```

```rust
struct Rectangle(f32, f32); //example of tuple struct
```

---

## Enum type

```rust
//simple enum
enum IpAddrKind {
    V4,
    V6,
}
//usage
let four = IpAddrKind::V4;
let six = IpAddrKind::V6;
```

```rust
enum IpAddr {
        V4(u8, u8, u8, u8),
        V6(String),
}
let home = IpAddr::V4(127, 0, 0, 1);
let loopback = IpAddr::V6(String::from("::1"));
```

---

## Functions

```rust
fn main() {
    println!("Hello, world!");

    another_function();
}

fn another_function() {
    println!("Another function.");
}
```

---

## Traits

```rust
// Trait example
trait Area: Debug {
  fn get_area(&self) -> f32;
}
```

```rust
//Trait implementation
impl Area for Square {
  fn get_area(&self) -> f32 {
    self.0 * self.0
  }
}
```

---

## Control flow: if

```rust
fn main() {
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {}", number);
}
```

---

## Control flow: for

```rust
fn main() {
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }
}

There is also: loop and while support.
```

---

## Control flow: pattern match

```rust
fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
}

let five = Some(5);
let six = plus_one(five);
let none = plus_one(None);
```

---

## Understanding Ownership - Key rust feature

---

## Ownership rules

- Each value in Rust has a variable that’s called its owner.
- There can only be one owner at a time.
- When the owner goes out of scope, the value will be dropped.

```rust
// Example
let s1 = String::from("hello");
let s2 = s1;

println!("{}, world!", s1);
```

---

## Ownership

```log
cargo run
   Compiling ownership v0.1.0 (file:///projects/ownership)
error[E0382]: borrow of moved value: `s1`
 --> src/main.rs:5:28
  |
2 |     let s1 = String::from("hello");
  |         -- move occurs because `s1` has type `std::string::String`, which does not implement the `Copy` trait
3 |     let s2 = s1;
  |              -- value moved here
4 |
5 |     println!("{}, world!", s1);
  |                            ^^ value borrowed here after move

error: aborting due to previous error
```

---

## Take ownership

```rust
fn main() {
    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it’s okay to still
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

L
```

---

## References and Borrowing

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

---

## Modify borrowed data

```rust
fn main() {
    let s = String::from("hello");

    change(&s);
}

fn change(some_string: &String) {
    some_string.push_str(", world");
}
// Compile error: cannot borrow `*some_string` as mutable,
//as it is behind a `&` reference
```

---

## Mutable References

```rust
//Example 1
fn main() {
    let mut s = String::from("hello");
    change(&mut s);
}
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
```

```rust
// Example 2
let mut s = String::from("hello");

let r1 = &mut s;
let r2 = &mut s;

println!("{}, {}", r1, r2);
```

---

## Result example 2

```log
$ cargo run
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

error: aborting due to previous error

For more information about this error, try `rustc --explain E0499`.
error: could not compile `ownership`.

To learn more, run the command again with --verbose
```

```rust
  let mut s = String::from("hello");
  {
      let r1 = &mut s;
  } // r1 goes out of scope here, so we can make a new reference with no problems.
  let r2 = &mut s;
```

---

## combining mutable and immutable references

```rust
let mut s = String::from("hello");

let r1 = &s; // no problem
let r2 = &s; // no problem
let r3 = &mut s; // BIG PROBLEM

println!("{}, {}, and {}", r1, r2, r3);
```

```log
cannot borrow `s` as mutable because it is also borrowed as
immutable
 --> src/main.rs:6:14
  |
4 |     let r1 = &s; // no problem
  |              -- immutable borrow occurs here
5 |     let r2 = &s; // no problem
6 |     let r3 = &mut s; // BIG PROBLEM
  |              ^^^^^^ mutable borrow occurs here
7 |
8 |     println!("{}, {}, and {}", r1, r2, r3);
  |                                -- immutable borrow later used
                                      here

```

---

## Lifetimes

- Every reference in Rust has a lifetime
- Is the scope for which that reference is valid
- Most of the time, lifetimes are implicit and inferred
- The main aim of lifetimes is to prevent dangling references

---

## Dangling References with Lifetimes

```rust
//this code does not compile
 {
    let r;
    {
        let x = 5;
        r = &x;
    }
    println!("r: {}", r);
  }
```

```log
 Compiling chapter10 v0.1.0 (file:///projects/chapter10)
error[E0597]: `x` does not live long enough
  --> src/main.rs:7:17
   |
7  |             r = &x;
   |                 ^^ borrowed value does not
                        live long enough
8  |         }
   |         - `x` dropped here while still borrowed
9  |
10 |         println!("r: {}", r);
   |                           - borrow later used here

error: aborting due to previous error
For more information about this error, try
`rustc --explain E0597`.
error: could not compile `chapter10`.
To learn more, run the command again with --verbose.
```

