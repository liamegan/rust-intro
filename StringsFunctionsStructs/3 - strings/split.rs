fn main() {
  let name = String::from("Liam Egan");
  let parts: Vec<&str> = name.split_whitespace().collect(); // This is how you split a string into parts.
  // split_whitespace is a method that splits a string into parts based on whitespace.
  // It is lazy and returns an iterator. You can collect the iterator into a vector.
  println!("{}", parts[0]);
}