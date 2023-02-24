// when data is bound by same immutably, it freezes
// so it can't be modified until it run out of scope

fn main() {
  let mut one = 1;
  {
    let one = one;
    one = 2;
  }
  one = 3;
}
