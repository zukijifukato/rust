// implicit casting is not allowed in Rust for
// for primitive types

// also cannot convert between different types
// like float cannot be converted into char

// but explicit conversions are allowed

fn main() {
  let x = 12.34;
  // let z: u8 = x; Error
  let y = x as i32;
  println!("x = {}, y = {}", x, y/* , z */);


  // also can only convert if the value is in range
  // println!("{}", 1000 as u8); Error

  // can perform c type conversions using unsafe though
  // unsafe {
    // let x = 300.0_f32.to_int_unchecked::<u8>();
  //}
}
