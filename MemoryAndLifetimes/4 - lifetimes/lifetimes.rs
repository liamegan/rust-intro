// https://app.rust-for-js.dev/posts/12-lifetimes/

struct Person<'a> {
  name: String,
  location: &'a String
}

fn main() { 
  let vancouver = String::from("Vancouver");
  let me = Person {
    name: String::from("Liam"),
    location: &vancouver
  };
  let alex = Person {
    name: String::from("Alex"),
    location: &vancouver
  };

  println!("Hello, {} from {}!", me.name, me.location); 
  println!("Hello, {} from {}!", alex.name, alex.location);
}

// If String implemented the Copy trait, this would just work because it would be automatically copied (cloned) under the hood 
// instead of moved.

// There's one more aspect of memory safety that the Rust compiler tries to ensure and this is the lifetime of a value. 
// Typically the lifetime of a value is its surrounding scope.
/*
fn main() {
    let x = 1;
    {
        let y = 2;
    }
}
*/
// In the above snippet the lifetime of x is the entire main function, while the lifetime of y is the inner block. The Rust 
// compiler will ensure that the lifetime of a value is not exceeded. This is done by the borrow checker which ensures that 
// references to a value are valid. The borrow checker will ensure that a reference to a value does not outlive the value 
// itself. This is done by the use of lifetimes

// A lifetime annotation looks like 'a and it is typical to use a single lowercase letter.
// What the struct above is telling us is that the Person struct's lifetime is now a function of its own scope and also the
// scope of the location reference. This means that the Person struct cannot outlive the location reference.

// An important point to remember is that you are not changing the behaviour or the lifetime of a value by annotating it
// with a lifetime. You are simply telling the Rust compiler that the lifetime of the value is a function of the lifetime
// of another value. In this case the Person struct's lifetime is a function of the location reference's lifetime.