//! Module: czmq-error

use std::borrow::Borrow;
use std::convert::{From, Into};
use std::error;
use std::ffi::NulError;
use std::fmt::{Display, Formatter, Result};
use std::str::Utf8Error;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ErrorKind {
    InvalidPath,
    NonZero,
    NullPtr,
    StringConversion,
}

#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
    cause: Box<error::Error>,
}

impl Error {
    pub fn new<E>(kind: ErrorKind, error: E) -> Error
        where E: Into<Box<error::Error>> {
        Error {
            kind: kind,
            cause: error.into(),
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self.kind {
            ErrorKind::InvalidPath => write!(f, "File path was invalid: {}", self.cause),
            ErrorKind::NonZero => write!(f, "CZMQ returned non-zero code: {}", self.cause),
            ErrorKind::NullPtr => write!(f, "CZMQ returned null pointer: {}", self.cause),
            ErrorKind::StringConversion => write!(f, "String conversion error: {}", self.cause),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match self.kind {
            ErrorKind::InvalidPath => "File path was invalid",
            ErrorKind::NonZero => "CZMQ returned non-zero code",
            ErrorKind::NullPtr => "CZMQ returned null pointer",
            ErrorKind::StringConversion => "Could not convert string to required type",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        Some(self.cause.borrow())
    }
}

impl From<NulError> for Error {
    fn from(ne: NulError) -> Error {
        Error::new(ErrorKind::StringConversion, ne)
    }
}

impl From<Utf8Error> for Error {
    fn from(u8e: Utf8Error) -> Error {
        Error::new(ErrorKind::StringConversion, u8e)
    }
}
