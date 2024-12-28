fn main() {
  // You can use if else branches as expressions
  let year = 1991;

  let generation = if year >= 1946 && year < 1965 {
    "Boomers"
  } else if year >= 1965 && year < 1981 {
    "Gen X"
  } else if year >= 1981 && year < 1997 {
    "Millennials"
  } else if year >= 1997 {
    "Gen Z"
  } else {
    "Too old or too young"
  };

  println!("Hello, {}!", generation);
}

// Rust is an expression based language, meaning that most things are expressions that return a value
// Therefore there is no ternary operator in Rust
// This works because if else branches are treated expressions that return a value
// The let generation works like any other assignment statement with the expression set to a nested set of conditionals.