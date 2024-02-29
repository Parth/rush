use std::io;

use shell_words::ParseError;

pub type Res<T> = Result<T, Error>;

#[derive(Debug)]
pub struct Error {
    pub msg: String,
}

impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Self {
            msg: value.to_string(),
        }
    }
}

impl From<&str> for Error {
    fn from(value: &str) -> Self {
        Self {
            msg: value.to_string(),
        }
    }
}

impl From<ParseError> for Error {
    fn from(value: ParseError) -> Self {
        Self {
            msg: value.to_string(),
        }
    }
}
