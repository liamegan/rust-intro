#[derive(Debug)]
enum HSL {
  Hue,
  Saturation,
  Lightness,
}

#[derive(Debug)] // This is a "derive" attribute that tells Rust to automatically create the Debug implementation for the enum.
enum Colour { // Define an enum called Colour
  Red, // This is a variant of the Colour enum
  Green,
  Blue,
  Rgb(u8, u8, u8), // This variant has three u8 values
  Hsl(HSL), // This variant has a value of the HSL enum
}

fn main() {
  let mut colour = Colour::Rgb(230, 230, 250); // Colour::Red; // The value of red is set to an instance of Red from the Colour enum. This is how you access enum variants.
  colour = Colour::Hsl(HSL::Hue);
  println!("Apples are: {:?}", colour);
}

// Enum variants can have values of different types too. In fact, you can even nest them as include enums within enums.

// A fun exercise might be to properly represent the three values for HSL using two more enums: a degrees enum (the hue is typically demarcated in degrees between 0 and 360) and a percentage enum that only allows values between 0 and 100.