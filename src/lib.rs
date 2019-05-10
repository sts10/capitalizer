pub mod downcase;
use downcase::downcase_this;

pub fn upcase_this(s: String) -> String {
    downcase_this(s.to_ascii_uppercase())
}
