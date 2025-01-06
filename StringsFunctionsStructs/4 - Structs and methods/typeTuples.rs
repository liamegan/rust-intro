#[derive(Debug)]
struct UserId(u32); // wrapper around a u32, but now distinct from plain u32

#[derive(Debug)]
struct ProductId(u32);

fn get_user_data(user_id: UserId) -> String {
  format!("User ID: {}", user_id.0)
}

fn main() {
  let user = UserId(42);
  let product = ProductId(99);

  // let data = get_user_data(product); // This will not compile because the function expects a UserId, not a ProductId.
  let data = get_user_data(user);

  println!("{}", data);
}