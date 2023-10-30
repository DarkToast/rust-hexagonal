use crate::domain::failure::Failure;

pub fn assert_result_ok<T>(result: Result<T, Failure>) -> T {
    match result {
        Ok(current) => current,
        Err(_) => panic!("Expected result OK, but was a failure."),
    }
}

pub fn assert_result_err<T>(result: Result<T, Failure>) -> Failure {
    match result {
        Ok(_) => panic!("Expected result failed, but was ok."),
        Err(failure) => failure,
    }
}
