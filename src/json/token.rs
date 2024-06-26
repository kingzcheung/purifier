use std::fmt::Display;

/// 对于Json的token有：
/// `,`, `:`, `{`, `}`, `[`, `]`, `String`, `Number`, `Boolean`, `Null`
#[derive(Debug, PartialEq)]
pub enum Token {
    Comma,
    Colon,
    BracketOn,
    BracketOff,
    BraceOn,
    BraceOff,
    String(String),
    Number(f64),
    Boolean(bool),
    Null,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Comma => write!(f, "comma"),
            Token::Colon => write!(f, "colon"),
            Token::BracketOn => write!(f, "bracket on"),
            Token::BracketOff => write!(f, "bracket off"),
            Token::BraceOn => write!(f, "brace on"),
            Token::BraceOff => write!(f, "brace off"),
            Token::String(_) => write!(f, "string"),
            Token::Number(_) => write!(f, "number"),
            Token::Boolean(_) => write!(f, "boolean"),
            Token::Null => write!(f, "null"),
        }
    }
}