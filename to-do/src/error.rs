use std::fmt::{Display, Formatter};
use std::io::ErrorKind;
use std::num::ParseIntError;
use std::{fmt, io, num, result};

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    ParseSequence(num::ParseIntError),
    InvalidSequence,
    MissingDescription,
    ParseTask(String),
}

impl From<num::ParseIntError> for Error {
    fn from(err: ParseIntError) -> Self {
        Self::ParseSequence(err)
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Self::Io(err)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Error::ParseSequence(_) => write!(f, "Input sequence is not a number"),
            Error::InvalidSequence => write!(f, "Input sequence number does not exist"),
            Error::MissingDescription => write!(f, "Missing task description"),
            Error::ParseTask(_) => write!(f, "Error parsing to-do(s)"),
            Error::Io(err) if err.kind() == ErrorKind::NotFound => {
                write!(f, "Error opening file: file not found")
            }
            Error::Io(err) if err.kind() == ErrorKind::PermissionDenied => {
                write!(f, "Error opening file: permission denied")
            }
            Error::Io(err) => write!(f, "{}", err.to_string()),
        }
    }
}
