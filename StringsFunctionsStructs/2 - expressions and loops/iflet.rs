#[derive(Debug)] // This is a "derive" attribute that tells Rust to automatically create the Debug implementation for the enum.
enum Status {
  Connected,
  Disconnected,
  Failure(String)
}
fn main() {
  let status = Status::Failure(String::from("Couldn't resolve the hostname"));
  // If let checks whether the value of `status`` matches the pattern `Status::Failure(err)`.
  // If it does, the value of the associated data is bound the value of `err`.
  // Borrowing the value with `&` prevents taking ownership of the value.
  if let Status::Failure(err) = &status {
    println!("Error: {}", err);
  }
  println!("Status: {:?}", status);
}

// Match syntax may be more explicit, but if let is more concise and idiomatic.