use core::fmt;
use std::error::Error;

#[derive(Debug)]
pub enum EmailParserError {
    IsNullValue,
    IsNotString,
    IsInvalid(String),
}

impl Error for EmailParserError {}
impl fmt::Display for EmailParserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::IsNullValue => write!(f, "The email field is null"),
            Self::IsNotString => write!(f, "The email field is not an String"),
            Self::IsInvalid(email) => write!(f, "The email: \"{}\" is invalid", email),
        }
    }
}

#[derive(Debug)]
pub enum ContentParserError {
    IsNullValue,
    IsNotString,
}

impl Error for ContentParserError {}
impl fmt::Display for ContentParserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::IsNullValue => write!(f, "The content field is null"),
            Self::IsNotString => write!(f, "The content field is not an String"),
        }
    }
}
