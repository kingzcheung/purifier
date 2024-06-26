use thiserror::Error;

use super::token::Token;

#[derive(Debug,Error)]
pub enum JsonError {
    #[error("Invalid label")]
    InvalidLabel(String),
    #[error("Invalid token {0}")]
    UnexpectedToken(Token),
}