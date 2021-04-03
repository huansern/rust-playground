use std::result;

pub type Result<T> = result::Result<T, Error>;

pub enum Error {
    InvalidSequence,
    ParseTask(String),
}
