type uint8_t = u8; // avoid non camel case types
                   // just added here for cool c type

fn main() {
  let x = 1 as uint8_t;

  println!("x: {}", x);
}
