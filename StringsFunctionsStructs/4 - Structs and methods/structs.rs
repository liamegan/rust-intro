// All fields in a struct must have a type.
#[derive(Debug)]
struct Person {
  name: String,
  age: u8
}

fn main() {
  let me = Person {
    name: String::from("Liam"),
    age: 45
  };
  let me_next_year = Person {
    age: 46,
    ..me
  };
  // If we want to borrow fields, we'd need to introduce the field as a borrowed field in the original struct.
  // And set up a lifetime annotation.
  // Or we need to clone the original struct, setting up a derive clonbe and then ..me.clone()
  // println!("{:?}", me);
  println!("{:?}", me_next_year);
}