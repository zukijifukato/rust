use std::mem;

fn main() {
  let xs: [i32; 5] = [1, 2, 3, 4, 5];

  // println!("{:?}", xs);

  let ys: [i32; 500] = [0; 500];
  // println!("{:?}", ys);

  // println!("number of elements in array: {}", xs.len());
  //println!("array occupies {} bytes", mem::size_of_val(&ys));
  println!("{}", xs[5]);
}
