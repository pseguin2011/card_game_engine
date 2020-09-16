use std::{error as e, fmt};

#[derive(Debug)]
pub enum CardGameError {
    IncorrectCardNumberRequest,
}


impl fmt::Display for CardGameError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CardGameError::IncorrectCardNumberRequest => write!(f, "Too Many Cards"),
        }
    }
}
impl e::Error for CardGameError {
    fn description(&self) -> &str {
        match *self {
            CardGameError::IncorrectCardNumberRequest => "Too Many Cards",
        }
    }
}
