fn main() {
  let mut product = ("iPhone 12 Pro Max", 1099, true); // Tuple, note the bracket syntax
  let (name, _, available) = product; // Destructuring a tuple, _ is a placeholder for a value we don't care about
  println!("Product: {} is available: {}, other: {}", name, available, product.1); // Accessing a tuple by index
}

// There is a special Tuple in Rust called the unit tuple. () represents the unit tuple. It has no values in it. This is what is returned by functions that return "nothing" in Rust.