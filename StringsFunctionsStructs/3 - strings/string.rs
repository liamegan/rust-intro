// String is the thing you typically want: a mutable, resizing data strcutre that can store strings.
fn main() {
  let mut name = String::new(); // This is how you create a new String.
  name.push_str("Liam"); // This is how you append a string to a String.
  // let mut name = String::from("Liam"); // Functionally equivalent to the two lines above.
  let firstname = String::from("Liam");
  let lastname = String::from("Egan");
  name = firstname + " " + &lastname; // This is how you concatenate strings.
  // You can't add strings together. You can only add a string slice to a string.
  // The + operator uses the add method, which takes ownership of the first string and a reference to the second string.
  // &string is a &String, a reference to a String. The compiler coerces the &String to a &str.
  println!("{}", name);
}

// Aside: :: is used for namespace-level access while . is used for instance-level access.
// :: us used for static/associated functions and constants.
// . is used for instance methods and fields.