fn main() {
  let one = 1;
  println!("let is great {}", one);

  let immut_one = 1;
  let mut mut_one = 1;
  println!("Immutable: {}, Mutable: {}", immut_one, mut_one);
  mut_one = 2;
  println!("Immutable: {}, Mutable: {}", immut_one, mut_one);

}
