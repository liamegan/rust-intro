fn sum(x: u64, y: u64) -> u64 {
  x+y // The last expression in a function is implicitly returned and does not need a semicolon (because it is an expression, not a statement)
}

fn main() {
  let result = sum(10, 5);
  println!("The sum of 10 and 5 is: {}", result);
}

// A function without a return value in Rust implicitly returns the unit tuple: ()