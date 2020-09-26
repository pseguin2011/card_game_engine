use std::{error as e, fmt};

#[derive(Debug)]
pub enum DefaultCardGameError {
    IncorrectCardNumberRequest,
    DeckEmpty,
}

impl fmt::Display for DefaultCardGameError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DefaultCardGameError::IncorrectCardNumberRequest => write!(f, "Too Many Cards were requested"),
            DefaultCardGameError::DeckEmpty => write!(f, "The deck is empty"),
        }
    }
}
impl e::Error for DefaultCardGameError {
    fn description(&self) -> &str {
        match *self {
            DefaultCardGameError::IncorrectCardNumberRequest =>  "Too Many Cards were requested",
            DefaultCardGameError::DeckEmpty => "The deck is empty",
        }
    }
}