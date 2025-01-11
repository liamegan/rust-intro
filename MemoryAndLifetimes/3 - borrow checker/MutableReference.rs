// You can only create a single mutable reference to a value
// If there is a mutable reference to a value, you cannot have any immutable references to it
fn scream(text: &mut String) {
  text.push_str("!");
}
fn main() {
  let mut me = String::from("Liam");
  let mut name = &mut me;
  scream(&mut me);
  println!("{}", name);
}