// shadowing can be done in local scope to
// override the global variable in local scope
// or can even override the variable in same scope

fn main() {
  let one = 1;
  {
    println!("one: {}", one);
    let one = "one";
    println!("one: {}", one);
  }

  let one = 'i';
  println!("one: {}", one);
}
