#[derive(Debug)]
struct Point(f32, f32, f32); // A 3D point with x and y coordinates.

fn main() {
  let origin = Point(0.,0.,0.);

  println!("The origin is at: {:?}", origin);
}