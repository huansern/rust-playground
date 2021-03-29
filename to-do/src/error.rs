use std::{io, result};

pub type Result<T> = result::Result<T, Error>;

pub enum Error {
    Io(io::Error),
    InvalidSequence,
    ParseTask(String),
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Self::Io(err)
    }
}
