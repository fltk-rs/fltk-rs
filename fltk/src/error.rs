use std::convert::From;
use std::string::FromUtf8Error;
use std::{fmt, io};

/// Error types returned by fltk-rs + wrappers of std errors
#[derive(Debug)]
#[non_exhaustive]
pub enum FltkError {
    /// i/o error
    IoError(io::Error),
    /// Utf-8 conversion error
    Utf8Error(FromUtf8Error),
    /// Null string conversion error
    NullError(std::ffi::NulError),
    /// Internal fltk error
    Internal(FltkErrorKind),
    /// Error using an erroneous env variable
    EnvVarError(std::env::VarError),
    /// Parsing error
    ParseIntError(std::num::ParseIntError),
    /// Unknown error
    Unknown(String),
}

unsafe impl Send for FltkError {}
unsafe impl Sync for FltkError {}

/// Error kinds enum for `FltkError`
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum FltkErrorKind {
    /// Failed to run the application
    FailedToRun,
    /// Failed to initialize the multithreading
    FailedToLock,
    /// Failed to set the general scheme of the application
    FailedToSetScheme,
    /// Failed operation, mostly unknown reason!
    FailedOperation,
    /// System resource (file, image) not found
    ResourceNotFound,
    /// Image format error when opening an image of an unsupported format
    ImageFormatError,
    /// Error filling table
    TableError,
    /// Error due to printing
    PrintError,
    /// Invalid color
    InvalidColor,
}

impl std::error::Error for FltkError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            FltkError::IoError(err) => Some(err),
            FltkError::NullError(err) => Some(err),
            _ => None,
        }
    }
}

impl fmt::Display for FltkError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            FltkError::IoError(ref err) => err.fmt(f),
            FltkError::NullError(ref err) => err.fmt(f),
            FltkError::Internal(ref err) => write!(f, "An internal error occurred {:?}", err),
            FltkError::EnvVarError(ref err) => write!(f, "An env var error occurred {:?}", err),
            FltkError::Utf8Error(ref err) => {
                write!(f, "A UTF8 conversion error occurred {:?}", err)
            }
            FltkError::ParseIntError(ref err) => {
                write!(f, "An int parsing error occurred {:?}", err)
            }
            FltkError::Unknown(ref err) => write!(f, "An unknown error occurred {:?}", err),
        }
    }
}

impl From<io::Error> for FltkError {
    fn from(err: io::Error) -> FltkError {
        FltkError::IoError(err)
    }
}

impl From<std::ffi::NulError> for FltkError {
    fn from(err: std::ffi::NulError) -> FltkError {
        FltkError::NullError(err)
    }
}

impl From<std::env::VarError> for FltkError {
    fn from(err: std::env::VarError) -> FltkError {
        FltkError::EnvVarError(err)
    }
}

impl From<std::string::FromUtf8Error> for FltkError {
    fn from(err: std::string::FromUtf8Error) -> FltkError {
        FltkError::Utf8Error(err)
    }
}

impl From<std::num::ParseIntError> for FltkError {
    fn from(err: std::num::ParseIntError) -> FltkError {
        FltkError::ParseIntError(err)
    }
}