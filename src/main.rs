mod downcase;
use crate::downcase::downcase_this;
fn main() {
    let greeting = "Hello, world".to_string();
    let lowercased_greeting = downcase_this(greeting);
    println!("{}", lowercased_greeting);
}
