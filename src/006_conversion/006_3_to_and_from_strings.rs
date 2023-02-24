// rather than implementing ToString trait to 
// convert any type to string we implement
// fmt::Display trait to convert to string
// also to print the type

// we can parse string to any type as long as
// FromString trait is implemented for that type

use std::fmt;

struct Circle {
  radius: i32,
}

impl fmt::Display for Circle {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "Circle of radius {}", self.radius)
  }
}

fn main() {
  let circle = Circle { radius: 6 };
  println!("{}", circle.to_string());

  let five: i32 = "5".parse().unwrap();
  let ten = "10".parse::<i32>().unwrap(); // turbofish syntax
  // turbofish is anything of type ::<SomeType>

  println!("Sum: {}", five + ten);
}
