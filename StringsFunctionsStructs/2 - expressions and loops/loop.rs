fn main() {
  // Infinite loop, think of it as a while(true) loop
  // loop {
  //   println!("Hello, world!");
  // }

  let mut counter = 0;
  // just like with if statements, you can use loop as an expression
  let opt = loop {
    // counter++; // Not a valid postfix operator
    counter += 1;
    println!("Hello, world! {}", counter);
    if counter>=10 {
      break counter; // break will retrun the value of counter
    }
  }; // Statement ends with a semicolon
  println!("Counter: {}", opt);
}