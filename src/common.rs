use std::error;
use std::fmt;

pub type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug)]
pub struct AnswerNotFound;

impl error::Error for AnswerNotFound {}

impl fmt::Display for AnswerNotFound {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "answer not found")
    }
}