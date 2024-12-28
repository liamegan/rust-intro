// In Rust, pattern matcing is like switch case in other languages
enum Status {
  Connected,
  Disconnected,
  Failure(String)
}
fn main() {
  let lan = Status::Failure(String::from("Could not contact the DHCP server"));
  // match is *exhaustive*, meaning that you need to cover all possible cases
  // This prevents a whole categoy of runtime panics that are common in other languages
  match &lan { // The & is a reference to the value, so that we don't take ownership of it
    Status::Disconnected => {
      println!("Connection lost");
    }
    Status::Failure(error) => {
      println!("Error: {}", error);
    }
    _ => {} // Catch all other cases
  }
  // Match is also an expression, meaning that it returns a value
  let status = match lan {
    Status::Connected => "Connected",
    Status::Disconnected => "Disconnected",
    Status::Failure(error) => {
      println!("Error: {}", error);
      "Failed"
    }
  };
  println!("Status: {}", status);
}