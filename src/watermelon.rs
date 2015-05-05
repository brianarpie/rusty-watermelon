use std::fmt::{self, Display, Formatter, Result};

struct Watermelon {
  color: &'static str,
  seeds: u16
}

impl Display for Watermelon {
  fn fmt(&self, f: &mut Formatter) -> Result {
    write!(f, "(color:{}, seeds:{})", self.color, self.seeds)
  }
}

fn main() {
  let mut bucket = [
    Watermelon { color: "red", seeds: 24 },
    Watermelon { color: "blue", seeds: 56 },
    Watermelon { color: "green", seeds: 106 }
  ];
  println!("{}", bucket[2]);
}


