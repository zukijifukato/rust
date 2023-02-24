#![allow(dead_code)]
#![allow(unused_variables)]

struct Unit; // unit struct
struct Pair(i32, f32); // tuple struct


#[derive(Debug, Copy, Clone)]
struct Point {
  x: f32,
  y: f32,
}

// struct inside a struct
#[derive(Debug, Copy, Clone)]
struct Rectangle {
  top_left: Point,
  bottom_right: Point,
}

fn square(p: Point, h: f32) -> Rectangle {
  Rectangle { top_left: p, bottom_right: Point { x: p.x + h, y: p.y + h } }
}

impl Rectangle {
  fn area(&self) -> f32 {
    let Rectangle { top_left: Point { x: a, y: b }, bottom_right: Point { x: c, y: d } } = *self;
    let length = c - a;
    let width = d - b;
    length * width
  }
}

fn main() {
  let a = Point { x: 1.0, y: 2.0 };
  let b = Point{x: 5.0, y: 6.0};
  let rect = Rectangle {
    top_left: a,
    bottom_right: b,
  };
  // both Point and rectangle needs to
  // derive from Debug to print in debug format
  println!("{:?}", rect);

  let unit = Unit;

  let pair = Pair(1, 0.1);

  let x = 1;

  // println!("\nPairs: {}, {}", pair.0, pair.1);

  println!("Area : {}", rect.area());

  let sq = square(rect.top_left, 5.0);
  println!("\nSquare: {:?}", sq);
}
