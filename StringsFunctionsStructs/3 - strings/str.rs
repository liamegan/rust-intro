// Both arrays and strings in Javascript are mutable and indeterminate in length.
// In rust there are two string types, a str and a String. A str is a string slice, and a String is a heap-allocated string.
fn main() {
  let greeting: &str = r#"Hello, \" world!"#; // This is a string slice, a str type.
  // let greeting = "Hello, \\\" world!"; // These two are functionally equivalent.
  // &str is a reference to a string slice, which is a reference to a sequence of UTF-8 bytes stored in the program's binary. This is why it's immutable.
  // &str is optional.
  // The r before the string is a raw string literal. It allows you to write a string without having to escape special characters.
  // Surrounding the string with # signs allows you to add a double quote to the string without having to escape it.
  println!("{}", greeting);
}

// In rust, string literals must always be surrounded by double quotes.
// Single quotes are used for character literals.
// All rust string linterals can be multi-line.