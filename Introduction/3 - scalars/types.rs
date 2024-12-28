// scalar types are single value types
// Similar to in mathematics, scalar types in Rust are single value types. They are the building blocks of all other types in Rust.
// Just like scalars in maths are single values, and vectors (enums) are multiple.
fn main() {
  let _is_cool: bool = true;
  let _initial: char = 'L'; // Chars should be single quoted
  let _age: u64 = 45;
  let _net_worth: f32 = 1.5;

  let price = 128;
  let tax = 0.08;
  let total: f64 = f64::from(price) * (1.0 + tax);
  println!("Total: {} + {}% = {}", price, tax * 100.0, total);
}

/*
while one reason to use unsigned integers is to represent values beyond the range of signed integers (up to , there are other practical reasons to choose unsigned integers. 
Semantics:
Unsigned integers make it explicit that a value cannot be negative. This is useful for concepts that are inherently non-negative, such as:
Counts (e.g., number of items)
Indices in arrays
Sizes and lengths
*/