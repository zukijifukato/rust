// primitives can be converted to each other  through casting
// custom types conversion can be done using traits.
// generic conversion uses `From` and `Into`

use std::convert::From;

#[allow(dead_code)]
#[derive(Debug)]
struct Number {
  value: i32
}

impl From<i32> for Number {
  fn from(item: i32) -> Self {
    Number { value: item }
  }
}

fn main() {
  let name = "john";
  let name = String::from(name);
  println!("Name: {}", name);

  let num = Number::from(32);
  println!("Number: {:?}", num);

  let number = 40;
  let forty: Number = number.into();
  println!("Number: {:?}", forty);
}
