fn main() {
  let s = String::from("book");

  println!(
    "I have one {}, you ahve two {}",
    s,
    you_add_something_here
  );

}

//Add appropriate parameters, return values, and implementation to this function
fn pluralize(singular: String) -> String {
  singular + "s";
}