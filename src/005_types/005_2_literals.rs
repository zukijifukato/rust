fn main() {
  let x = 1u8; // can add type as suffix
  let y = 1; // type can depend on usage

  println!("size of x: {}", std::mem::size_of_val(&x)); // 1 byte
  println!("size of y: {}", std::mem::size_of_val(&y)); // 4 bytes
}
