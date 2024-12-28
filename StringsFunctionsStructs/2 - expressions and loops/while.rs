// While loops are almost identical to javascript while loops
fn main() {
  let mut counter = 0;
  let test = while counter < 10 {
    println!("Hello, world! {}", counter);
    counter += 1;
  };
  // println!("Counter: {}", test); // While loops are not a good use for expressions?
}