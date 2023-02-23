fn main() {
  /* scalar types */
  // i8, i16, ... i128, isize(based on architecture size)
  // same as above for unsigned (u8... usize)
  // f32, f64
  // char
  // bool
  // unit type () - empty tuple

  /* compound types */
  // [1, 2, 4] - arrays
  // (1, 2, "Hello") - tuples

  /* Shadowing */
  // overwritting outer block variable with new value
  // or withing the same block
  let one = 1;
  let mut one = 2;
  println!("{one}");
}
