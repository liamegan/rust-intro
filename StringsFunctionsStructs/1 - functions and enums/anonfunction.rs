fn main() {
  let sum = |x, y| x+y; // Anon functions don't need types, Rust can infer them
  let result = sum(10, 5);
  println!("The sum of 10 and 5 is: {}", result);
}