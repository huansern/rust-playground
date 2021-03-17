use std::error;
use std::fmt;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub struct Error {
    bucket: String,
    object: String,
    kind: Kind,
}

#[derive(Debug)]
pub enum Kind {
    BucketNotFound,
    ObjectAlreadyExist,
    ObjectNotFound,
    IO,
}

impl Error {
    pub fn new(kind: Kind, bucket: &str, object: &str) -> Self {
        Error {
            bucket: bucket.into(),
            object: object.into(),
            kind,
        }
    }
}

impl error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.kind {
            Kind::BucketNotFound => write!(f, "bucket {} not found", self.bucket),
            Kind::ObjectAlreadyExist => write!(
                f,
                "object {} already exist in {} bucket",
                self.object, self.bucket
            ),
            Kind::ObjectNotFound => write!(
                f,
                "object {} is not found in {} bucket",
                self.object, self.bucket
            ),
            Kind::IO => write!(
                f,
                "error writing object {} into {} bucket",
                self.object, self.bucket
            ),
        }
    }
}
