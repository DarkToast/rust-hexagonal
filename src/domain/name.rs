use regex::Regex;

use super::failure::Failure;
use super::validators::{assert_format, assert_length};

pub struct Name(String);

impl Name {
    // all german alphanumeric characters.
    pub fn new(value: &str) -> Result<Self, Failure> {
        let re_ger: Regex = Regex::new(r"^[a-zA-Z0-9üäößÄÖÜ]{0,20}$").unwrap();

        let length = assert_length("name", 1, 20, value);
        let format = assert_format("name", re_ger, value);

        match length.or(format) {
            Some(failure) => Err(failure),
            None => Ok(Name(String::from(value))),
        }
    }

    pub fn raw(&self) -> String {
        self.0.clone()
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::failure::Failure::{WrongFormat, TooLong};
    use crate::domain::name::Name;
    use crate::tests::assertions::{assert_result_ok, assert_result_err};
        
    #[test]
    fn a_name_is_correct() {
        let result = Name::new("Name123füä");
        let name = assert_result_ok(result);
        assert_eq!("Name123füä", name.raw());
    }

    #[test]
    fn a_name_is_too_long() {
        let result = Name::new("012345678901234567891");
        let name = assert_result_err(result);

        match name {
            TooLong { field, message } => {
                assert_eq!("name", field);
                assert_eq!("String must between 1 and 20. Is 21.", message);

            },
            _ => assert!(false, "Wrong failure type")
        }
    }

    #[test]
    fn a_name_is_wrong_format() {
        let result = Name::new("@#$%^");
        let name = assert_result_err(result);

        match name {
            WrongFormat { field, message } => {
                assert_eq!("name", field);
                assert_eq!("The string has not the correct format", message);

            },
            _ => assert!(false, "Wrong failure type")
        }
    }
}
