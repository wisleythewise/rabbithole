# `rabbithole_macro`

This crate provides a procedural macro, `#[rabbithole]`, that **debug-prints** intermediate expressions within a function. When a function is annotated with `#[rabbithole]`, any expression in that function is automatically wrapped in a `dbg!()` call, causing its value to be printed at runtime.

## How It Works

When you place `#[rabbithole]` above a function:

1. The function body is parsed into an Abstract Syntax Tree (AST).
2. Each statement containing an expression is wrapped in `dbg!()`.
3. The function then compiles to valid Rust code, but every expression is debug-printed.

This macro helps trace expressions without manually inserting numerous `dbg!()` calls.

## Example

Below is a `Point` struct with a method that calculates the distance between two points:

```rust
use rabbithole_macro::rabbithole;

struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    #[rabbithole]
    fn distance(&self, other: &Point) -> f32 {
        let y_delta = self.y - other.y;
        let x_delta = self.x - other.x;
        let dist = ((y_delta.pow(2) + x_delta.pow(2)) as f32).sqrt();
        return dist;
    }
}

fn main() {
    let point_1 = Point::new(1, 1);
    let point_2 = Point::new(2, 2);
    println!("distance between the points! {:?}", point_1.distance(&point_2));
}
```

```
[rabbithole_bin/src/main.rs:<line>] self.y - other.y = ...
[rabbithole_bin/src/main.rs:<line>] self.x - other.x = ...
[rabbithole_bin/src/main.rs:<line>] ((y_delta.pow(2) + x_delta.pow(2)) as f32).sqrt() = ...
distance between the points! ...
```
