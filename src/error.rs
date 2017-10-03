use nom;

use std::{error, fmt, io, result};

#[derive(Debug)]
pub enum Error {
    IOError(io::Error),
    ParseError(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::IOError(ref err)    => write!(f, "IO Error: {}", err),
            Error::ParseError(ref err) => write!(f, "Parse Error: {}", err),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::IOError(ref err)    => err.description(),
            Error::ParseError(ref err) => err,
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::IOError(ref err) => Some(err),
            Error::ParseError(_)    => None,
        }
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::IOError(err)
    }
}

impl<'a> From<nom::Err<&'a [u8], u32>> for Error {
    fn from(err: nom::Err<&[u8], u32>) -> Error {
        Error::ParseError(format!("{:?}", err))
    }
}

pub type Result<T> = result::Result<T, Error>;
