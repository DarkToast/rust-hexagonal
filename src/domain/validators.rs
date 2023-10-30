use regex::Regex;

use super::failure::Failure;

pub fn assert_length(field: &str, min: usize, max: usize, value: &str) -> Option<Failure> {
    if value.len() > max || value.len() < min {
        Some(Failure::TooLong { 
            field: String::from(field),
            message: format!("String must between {} and {}. Is {}.", min, max, value.len()) 
        })
    } else {
        None
    }
}

pub fn assert_format(field: &str, regex: Regex, value: &str) -> Option<Failure> {
    if regex.is_match(value) {
        None
    } else {
        Some(Failure::WrongFormat { 
            field: String::from(field),
            message: String::from("The string has not the correct format") 
        })
    }
}