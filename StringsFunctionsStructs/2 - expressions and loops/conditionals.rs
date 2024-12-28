fn main() {
  let year = 2192;
  // Rust conditionals do not require parentheses
  // They also don't cast to boolean
  // Always remember to wriite conditional expressions that evaluate to a boolean
  // `if list.len() { }` will not work, adding `> 0` will make it work
  if year >= 1946 && year < 1965 {
    println!("Hello boomers!");
  } else if year >= 1965 && year < 1981 {
    println!("Hello Gen X!");
  } else if year >= 1981 && year < 1997 {
    println!("Hello Millennials!");
  } else if year >= 1997 {
    println!("Hello Gen Z!");
  } else {
    println!("Hello, you're too old or too young for this program!");
  }
}