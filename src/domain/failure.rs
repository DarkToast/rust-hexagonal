#[cfg_attr(test, derive(Debug))]
pub enum Failure {
    WrongFormat { field: String, message: String },
    TooLong { field: String, message: String },
}
