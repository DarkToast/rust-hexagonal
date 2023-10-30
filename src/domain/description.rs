use regex::Regex;

use super::{failure::Failure, validators::{assert_length, assert_format}};

pub struct Description(String);

impl Description {
    // all german alphanumeric characters including space, dots and commas.
    pub fn new(value: &str) -> Result<Self, Failure> {
        let re_ger: Regex = Regex::new(r"^([a-zA-Z0-9üäößÄÖÜ]+[a-zA-Z0-9üäößÄÖÜ \-\.,])*$").unwrap();
        
        let length = assert_length("description", 0, 120, value);
        let format = assert_format("description", re_ger, value);

        match length.or(format) {
            Some(failure) => Err(failure),
            None => Ok(Description(String::from(value)))
        }
    }

    pub fn raw(&self) -> String {
        self.0.clone()
    }
}


#[cfg(test)]
mod tests {
    use crate::domain::description::Description;
    use crate::domain::failure::Failure::{WrongFormat, TooLong};
    use crate::tests::assertions::{assert_result_err, assert_result_ok};

    #[test]
    fn a_name_is_correct() {
        let result = Description::new("Das ist eine schöne Beschreibung.");
        let name = assert_result_ok(result);
        assert_eq!("Das ist eine schöne Beschreibung.", name.raw());
    }

    #[test]
    fn a_name_is_too_long() {
        let too_long: String = (0..31).map(|_|"abcd").collect();

        let result = Description::new(too_long.as_str());
        let name = assert_result_err(result);

        match name {
            TooLong { field, message } => {
                assert_eq!("description", field);
                assert_eq!("String must between 0 and 120. Is 124.", message);

            },
            _ => assert!(false, "Wrong failure type")
        }
    }

    #[test]
    fn a_name_starts_with_blank() {
        let result = Description::new(" Hallo");
        let name = assert_result_err(result);

        match name {
            WrongFormat { field, message } => {
                assert_eq!("description", field);
                assert_eq!("The string has not the correct format", message);

            },
            _ => assert!(false, "Wrong failure type")
        }
    }

    #[test]
    fn a_name_starts_with_line() {
        let result = Description::new("- Hallo");
        let name = assert_result_err(result);

        match name {
            WrongFormat { field, message } => {
                assert_eq!("description", field);
                assert_eq!("The string has not the correct format", message);

            },
            _ => assert!(false, "Wrong failure type")
        }
    }

    #[test]
    fn a_name_starts_with_dot() {
        let result = Description::new(". Hallo");
        let name = assert_result_err(result);

        match name {
            WrongFormat { field, message } => {
                assert_eq!("description", field);
                assert_eq!("The string has not the correct format", message);

            },
            _ => assert!(false, "Wrong failure type")
        }
    }

    #[test]
    fn a_name_is_wrong_format() {
        let result = Description::new("@#$%^");
        let name = assert_result_err(result);

        match name {
            WrongFormat { field, message } => {
                assert_eq!("description", field);
                assert_eq!("The string has not the correct format", message);

            },
            _ => assert!(false, "Wrong failure type")
        }
    }
}
