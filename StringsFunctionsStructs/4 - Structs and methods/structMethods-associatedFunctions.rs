// These are link instanc methods in javascript. They are methods that are called on an instance of a struct.
#[derive(Debug)]
struct URL {
  protocol: String,
  hostname: String,
  pathname: String
}

// Associated functions are what we're used to calling static methods. These live on the struct and not on the instance.
// Associated functions are defined without a self parameter.
impl URL {
  fn toString(&self) -> String {
    format!("{}://{}/{}", self.protocol, self.hostname, self.pathname)
  }
  fn from(url: &str) -> URL {
    let string = String::from(url);
    let mut vec: Vec<&str> = string.split("://").collect(); // collect is a method that collects an iterator into a collection (vec?).
    let protocol = String::from(vec[0]);
    let rest = String::from(vec[1]);
    vec = rest.split("/").collect();
    let hostname = String::from(vec[0]);
    let path = String::from(vec[1]);
    URL {
      protocol,
      hostname,
      pathname: path
    }
  }
}

fn main() {
  let app = URL::from("HTTPS://wethecollective.com/insights/new");
  println!("{:?}", app);
}