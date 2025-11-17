---
theme: default
author: Learn Rust
---

# Learn Rust

A quick tour through Rust fundamentals

---

## Comments

Rust supports multiple comment styles:

```rust
// Line comments look like this
// and extend multiple lines

/* Block comments
   /* can be nested. */ */

/// Documentation comments support markdown
/// # Examples
/// ```
/// let five = 5
/// ```
```

---

## Functions

Functions use `fn` keyword with type annotations:

```rust
fn add2(x: i32, y: i32) -> i32 {
    // Implicit return (no semicolon)
    x + y
}
```

**Key points:**

- `i32` is a 32-bit signed integer
- Last expression without `;` is the return value
- Function parameters must have type annotations

---

## Variables

Rust has immutable bindings by default:

```rust
// Immutable binding
let x: i32 = 1;

// Type inference works most of the time
let implicit_x = 1;
let implicit_f = 1.3;

// Mutable variable
let mut mutable = 1;
mutable = 4;
mutable += 2;
```

---

## Numbers

Integer and float types with suffixes:

```rust
// Integer/float suffixes
let y: i32 = 13i32;
let f: f64 = 1.3f64;

// Arithmetic
let sum = x + y + 13;
```

---

## Strings

Two main string types in Rust:

```rust
// String slice (&str) - immutable view
let x: &str = "hello world!";

// String - heap-allocated, growable
let s: String = "hello world".to_string();

// String slice from String
let s_slice: &str = &s;

// Printing
println!("{} {}", f, x);
```

---

## Arrays and Vectors

Fixed-size arrays and dynamic vectors:

```rust
// Fixed-size array
let four_ints: [i32; 4] = [1, 2, 3, 4];

// Dynamic vector
let mut vector: Vec<i32> = vec![1, 2, 3, 4];
vector.push(5);

// Slice - immutable view
let slice: &[i32] = &vector;

// Debug printing
println!("{:?} {:?}", vector, slice);
```

---

## Tuples

Fixed-size sets of values of possibly different types:

```rust
// Tuple declaration
let x: (i32, &str, f64) = (1, "hello", 3.4);

// Destructuring
let (a, b, c) = x;
println!("{} {} {}", a, b, c); // 1 hello 3.4

// Indexing
println!("{}", x.1); // hello
```

---

## Structs

Custom data types with named fields:

```rust
struct Point {
    x: i32,
    y: i32,
}

let origin: Point = Point { x: 0, y: 0 };

// Tuple struct (unnamed fields)
struct Point2(i32, i32);
let origin2 = Point2(0, 0);
```

---

## Enums

Enums can have variants with or without data:

```rust
// Basic C-like enum
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

let up = Direction::Up;

// Enum with fields
enum OptionalI32 {
    AnI32(i32),
    Nothing,
}

let two: OptionalI32 = OptionalI32::AnI32(2);
```

---

## Generics

Type parameters for reusable code:

```rust
struct Foo<T> { bar: T }

enum Optional<T> {
    SomeVal(T),
    NoVal,
}
```

The standard library provides `Option<T>` for optional values, replacing null pointers.

---

## Methods

Functions associated with types:

```rust
impl<T> Foo<T> {
    // Borrowed self
    fn bar(&self) -> &T {
        &self.bar
    }

    // Mutably borrowed self
    fn bar_mut(&mut self) -> &mut T {
        &mut self.bar
    }

    // Consumed self
    fn into_bar(self) -> T {
        self.bar
    }
}
```

---

## Traits

Interfaces that define shared behavior:

```rust
trait Frobnicate<T> {
    fn frobnicate(self) -> Option<T>;
}

impl<T> Frobnicate<T> for Foo<T> {
    fn frobnicate(self) -> Option<T> {
        Some(self.bar)
    }
}

let foo = Foo { bar: 1 };
println!("{:?}", foo.frobnicate()); // Some(1)
```

---

## Pattern Matching

Powerful control flow with `match`:

```rust
let foo = OptionalI32::AnI32(1);
match foo {
    OptionalI32::AnI32(n) => println!("it's an i32: {}", n),
    OptionalI32::Nothing  => println!("it's nothing!"),
}
```

---

## Advanced Pattern Matching

Destructure and use guards:

```rust
struct FooBar { x: i32, y: OptionalI32 }
let bar = FooBar { x: 15, y: OptionalI32::AnI32(32) };

match bar {
    FooBar { x: 0, y: OptionalI32::AnI32(0) } =>
        println!("The numbers are zero!"),
    FooBar { x: n, y: OptionalI32::AnI32(m) } if n == m =>
        println!("The numbers are the same"),
    FooBar { x: n, y: OptionalI32::AnI32(m) } =>
        println!("Different numbers: {} {}", n, m),
    FooBar { x: _, y: OptionalI32::Nothing } =>
        println!("The second number is Nothing!"),
}
```

---

## For Loops

Iterate over arrays and ranges:

```rust
// Array iteration
let array = [1, 2, 3];
for i in array {
    println!("{}", i);
}

// Range iteration
for i in 0u32..10 {
    print!("{} ", i);
}
// prints: 0 1 2 3 4 5 6 7 8 9
```

---

## If Expressions

`if` can be used as an expression:

```rust
if 1 == 1 {
    println!("Maths is working!");
} else {
    println!("Oh no...");
}

// if as expression
let value = if true {
    "good"
} else {
    "bad"
};
```

---

## Loops

Multiple loop constructs:

```rust
// while loop
while condition {
    println!("Looping...");
    break  // Exit the loop
}

// Infinite loop
loop {
    println!("Hello!");
    break  // Must break explicitly
}
```

---

## Owned Pointers (Box)

`Box<T>` provides heap allocation with single ownership:

```rust
let mut mine: Box<i32> = Box::new(3);
*mine = 5; // dereference

// Ownership transfer (move)
let mut now_its_mine = mine;
*now_its_mine += 2;

println!("{}", now_its_mine); // 7
// println!("{}", mine); // Error! moved
```

When `Box` goes out of scope, memory is automatically deallocated.

---

## Immutable References

Borrowing without transferring ownership:

```rust
let mut var = 4;
var = 3;
let ref_var: &i32 = &var;

println!("{}", var);      // Still works!
println!("{}", *ref_var); // 3

// var = 5;      // Error! var is borrowed
// *ref_var = 6; // Error! immutable reference

ref_var; // Use the reference
var = 2; // Borrow ended, can mutate again
```

---

## Mutable References

Exclusive mutable access:

```rust
let mut var2 = 4;
let ref_var2: &mut i32 = &mut var2;
*ref_var2 += 2;

println!("{}", *ref_var2); // 6

// var2 = 2; // Error! var2 is mutably borrowed

ref_var2; // Use ends here
// Now var2 can be used again
```

**Key rule:** Either many immutable references OR one mutable reference.

---

## Memory Safety

Rust's borrow checker ensures:

- No use after free
- No double free
- No data races
- No dangling pointers

All at **compile time** with **zero runtime cost**.

---

## Next Steps

- Explore the [Rust Book](https://doc.rust-lang.org/book/)
- Try [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- Practice with [Rustlings](https://github.com/rust-lang/rustlings)
- Join the [Rust community](https://www.rust-lang.org/community)

---

## Thank You

Happy Rusting!
