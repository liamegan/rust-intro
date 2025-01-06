// https://app.rust-for-js.dev/posts/07-structs-and-methods/
// Tuple structs are just like tuples, but with names. They are useful when you want to give a tuple a name and make the tuple a distinct type.
#[derive(Debug)]
struct Celcius(i16); // This is a tuple struct. It has one field, an i16.

fn main(){
  let boiling = Celcius(100); // This is how you create a tuple struct.
}

// This reads better than writing this in a struct with a field called temperature.
// In use cases like these, often the type is used to enforce some kind of invariant or to make the code more readable.
// They can also be used to create distinct types that prevent misuse.