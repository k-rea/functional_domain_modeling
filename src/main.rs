mod domain;

struct NonEmptyString(String);

impl NonEmptyString {
    fn new(s: String) -> Result<Self, &'static str> {
        if s.is_empty() {
            Err("String must not be empty")
        } else {
            Ok(NonEmptyString(s))
        }
    }
}

fn main() {
}
