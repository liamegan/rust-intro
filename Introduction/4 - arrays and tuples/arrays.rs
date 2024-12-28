fn main() {
  // let top_scores: [u32; 3] = [292,100,140];
  let mut top_scores: [u32; 3] = [2; 3]; // This is a shorthand way to initialize an array with the same value. The [2; 3] means 2 repeated 3 times.
  top_scores[2] = 292;
  println!("Top scores({}): {:?}", top_scores.len(), top_scores); // The :? makes println! use the Debug "trait" instead of the default Display and lets you quickly print out values as you learn.
}