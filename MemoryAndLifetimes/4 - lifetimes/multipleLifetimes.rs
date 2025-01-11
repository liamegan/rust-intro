// https://app.rust-for-js.dev/posts/12-lifetimes/

use std::collections::HashMap;

#[derive(Debug)]
struct Person<'a> {
  name: &'a str,
  location: &'a str
}

fn getCity<'a, 'b>(code: &'a str, airport_codes: &'b HashMap<&'b str, &'b str>) -> &'b str {
  airport_codes.get(code).expect(format!("City not found for code: {}", code).as_str())
 }

fn main() { 
  let airport_codes = HashMap::from([
    ("YVR", "Vancouver"),
    ("YYZ", "Toronto"),
    ("YUL", "Montreal")
  ]);

  let me = Person {
    name: "Liam",
    location: "YVR"
  };
  let alex = Person {
    name: "Alex",
    location: "SYD"
  };

  println!("{:?}", airport_codes.get("YVR"));
  println!("{:?}", airport_codes.get("SYD"));
  println!("Hello, {} from {}!", alex.name, getCity(alex.location, &airport_codes));
}

// Notice how both the name and location fields have the same lifetime annotation. 
// They were created in the same scope and hence have the same lifetime.
// expect is function used to unwrap an Option and returns the value unwrapped from 
// a Some and panics if the returned value is None (with the message passed in as an 
// argument to expect)

// Sometimes it might be helpful to annotate relationships between lifetimes. 
// In the getCity function, we have two lifetimes 'a and 'b. 'a is the lifetime of
// the code parameter and 'b is the lifetime of the airport_codes parameter.
// The return type of the function is &'b str which means that the returned value
// will have the same lifetime as the airport_codes parameter. If we wanted to
// annotate the fact that we expect airport_codes to outlive code, we could do so
// by writing the function signature as 
// fn getCity<'a, 'b>(code: &'a str, airport_codes: &'b HashMap<&'b str, &'b str>) -> &'b str where 'b: 'a {
// This represents the fact that 'b must outlive 'a.

// Static lifetime
// The 'static lifetime is a special lifetime that represents the entire duration of the program.
// The most typical of these are variables declared with the static keyword. e.g.:
// static HELLO: &str = "Hello, world!";
// fn main() {
//   println!("{}", HELLO);
// }
// The static lifetime is inferred and does not need to be explicitly annotated.