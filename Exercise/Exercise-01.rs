fn main() {
  let s = String::from("book");

  // making a clone and storing it after pluralizing it
  let pl = pluralize(s.clone());

  println!(
    "I have one {}, you have two {}",
    s,
    // you_add_something_here
    pl
  );
}

//Add appropriate parameters, return values, and implementation to this function
fn pluralize(singular: String) -> String {
  singular + "s"
}