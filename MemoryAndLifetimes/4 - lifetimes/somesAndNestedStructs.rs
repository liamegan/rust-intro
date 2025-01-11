// https://app.rust-for-js.dev/posts/12-lifetimes/

use std::collections::HashMap;

#[derive(Debug)]
struct Person<'a> {
  name: Name,
  location: &'a String
}
#[derive(Debug)]
struct Name {
  first: String,
  middle: Option<String>,
  last: String
}

fn main() { 
  let airport_codes = HashMap::from([
    ("YVR", "Vancouver"),
    ("YYZ", "Toronto"),
    ("YUL", "Montreal")
  ]);

  let vancouver = String::from("Vancouver");
  let me = Person {
    name: Name {
      first: String::from("Liam"),
      middle: Some(String::from("Geoffrey")),
      last: String::from("Egan")
    },
    location: &vancouver
  };
  let alex = Person {
    name: Name {
      first: String::from("Alex"),
      middle: None,
      last: String::from("Verdecchia")
    },
    location: &vancouver
  };

  println!("{:?}", airport_codes.get("YVR"));
  println!("{:?}", airport_codes.get("SYD"));
  println!("Hello, {:?}!", me.name.middle);
}

// The get function returns an enum called Option. This enum can be either Some or None.
// To represent an optional value in Javascript, we set a variable to null or undefined
// and then run some checks to see if it is null or undefined.
// In Rust, we use the Option enum to represent optional values.
// When a value is present, we wrap it using the Some variant, and when it's not present, we use the None variant.