pub mod downcase;
use crate::downcase::downcase_this;

pub fn upcase_this(s: String) -> String {
    downcase_this(s.to_ascii_uppercase())
}
