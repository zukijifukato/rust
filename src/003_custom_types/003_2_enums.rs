#![allow(dead_code, unused_variables)] // crate level macro

#[derive(Debug)]
enum State {
  Open,
  Close,
}

// type alias like typescript
#[allow(dead_code)] // struct enum level macro
type DoorStatus = State;

// C like enums
enum Color {
  Red = 0xff0000,
  Green = 0x00ff00,
  Blue = 0x0000ff,
}

fn main() {
  use State::{Open, Close};
  let door_status = Close;

  match door_status {
    Open => println!("Door is open!"),
    Close => println!("Door is closed!")
  }

  println!("Roses are #{:x}", Color::Red as i32);
}
