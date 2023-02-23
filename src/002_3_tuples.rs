use std::fmt;

struct Matrix(f32, f32, f32, f32);

impl fmt::Display for Matrix {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "({} {})\n({} {})", self.0, self.1, self.2, self.3)
  }
}

fn transpose(mat: Matrix) -> Matrix {
  Matrix(mat.0, mat.2, mat.1, mat.3)
}

fn main() {
  let tuple = ("Hello", 1, 2);
  println!("{:?}", tuple);

  println!("{:?}", (5u32)); // a literal
  println!("{:?}", (5u32,)); // a tuple

  let tups = ("john", "NYC", 123);
  let (name, addr, phone) = tups;
  println!("{}, {}, {}", name, addr, phone);

  // exercise
  //implementing display format
  let mat = Matrix(1.1, 1.2, 2.1, 2.2);
  println!("\nMatrix: \n{}", mat);

  // matrix transpose
  println!("\nTranspose: \n{}", transpose(mat));
}
