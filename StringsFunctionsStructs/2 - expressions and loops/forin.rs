// The for in loop is a way to iterate over a collection of items. Apparently bread and butter.
fn main() {
  for planet in ["Mercury", "Venus", "Earth", "Mars", "Jupiter", "Saturn", "Uranus", "Neptune"] {
    println!("Hello, {}!", planet);
  }
}
// To make a collection iterable, you need to implement the IntoIterator trait.