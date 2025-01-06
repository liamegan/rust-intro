// These are link instanc methods in javascript. They are methods that are called on an instance of a struct.
struct URL {
  protocol: String,
  hostname: String,
  pathname: String
}

impl URL {
  fn toString(&self) -> String {
    format!("{}://{}/{}", self.protocol, self.hostname, self.pathname)
  }
}

fn main() {
  let url = URL {
    protocol: String::from("HTTPS"),
    hostname: String::from("wethecollective.com"),
    pathname: String::from("insights/new")
  };
  println!("{}", url.toString());
}