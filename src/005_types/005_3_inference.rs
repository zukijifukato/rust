fn main() {
  let x  = 1; // infers the type as int of 4 bytes
  let mut vec = Vec::new();
  vec.push(x); // infers that the type of vector is int of 4 bytes
  println!("{:?}", vec);
}
