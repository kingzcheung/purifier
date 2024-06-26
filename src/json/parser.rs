use std::collections::HashMap;


use super::tokenizer::Tokenizer;
use super::token::Token;
use super::value::Json;

pub struct Parser<'a> {
    tokenizer: Tokenizer<'a>,
}


impl<'a> Parser<'a> {
    pub fn new(s: &'a str) -> Self {
        Self {
            tokenizer: Tokenizer::new(s),
        }
    }

    pub fn parse(&mut self) -> Result<Json, super::error::JsonError> {
        let token = self.step();

        self.parse_from(token?)
    }

    fn step(&mut self) -> Result<Token, super::error::JsonError> {
        self.tokenizer.next().expect("Unexpected end of JSON!!!")
    }

    fn parse_array(&mut self) -> Result<Json, super::error::JsonError> {
        let mut array = Vec::new();

        match self.step()? {
            Token::BracketOff => return Ok(array.into()),
            token => array.push(self.parse_from(token)?),
        }

        loop {
            match self.step()? {
                Token::Comma => array.push(self.parse()?),
                Token::BracketOff => break,
                token => panic!("Unexpected token {:?}", token),
            }
        }

        Ok(array.into())
    }

    fn parse_object(&mut self) -> Result<Json, super::error::JsonError> {
        let mut object = HashMap::new();

        match self.step()? {
            Token::BraceOff => return Ok(object.into()),
            Token::String(key) => {
                match self.step()? {
                    Token::Colon => (),
                    token => panic!("Unexpected token {:?}", token),
                }
                let value = self.parse()?;
                object.insert(key, value);
            }
            token => panic!("Unexpected token {:?}", token),
        }

        loop {
            match self.step()? {
                Token::Comma => {
                    let key = match self.step()? {
                        Token::String(key) => key,
                        token => panic!("Unexpected token {:?}", token),
                    };
                    match self.step()? {
                        Token::Colon => {}
                        token => panic!("Unexpected token {:?}", token),
                    }
                    let value = self.parse()?;
                    object.insert(key, value);
                }
                Token::BraceOff => break,
                token => panic!("Unexpected token {:?}", token),
            }
        }

        Ok(object.into())
    }

    fn parse_from(&mut self, token: Token) -> Result<Json,super::error::JsonError> {
        match token {
            Token::Null => Ok(Json::Null),
            Token::String(s) => Ok(Json::String(s)),
            Token::Number(n) => Ok(Json::Number(n)),
            Token::Boolean(b) => Ok(Json::Boolean(b)),
            Token::BracketOn => self.parse_array(),
            Token::BraceOn => self.parse_object(),
            _ => panic!("Unexpected token: {:?}", token),
        }
    }
}