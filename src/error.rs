// Copyright 2024 Heath Stewart.
// Licensed under the MIT License. See LICENSE.txt in the project root for license information.

use std::{
    borrow::{Borrow, Cow},
    fmt,
    string::FromUtf8Error,
};

pub type Result<T> = std::result::Result<T, Error>;
pub type RawOsError = i32;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ErrorKind {
    InvalidData,
    Io,
    Os(RawOsError),
    Other,
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // cspell:ignore errno
        match self {
            ErrorKind::InvalidData => f.write_str("InvalidData"),
            ErrorKind::Io => f.write_str("Io"),
            ErrorKind::Os(errno) => write!(f, "Os({errno})"),
            ErrorKind::Other => f.write_str("Other"),
        }
    }
}

#[derive(Debug)]
pub struct Error {
    repr: Repr,
}

impl Error {
    /// Constructs a new `Error` boxing another [`std::error::Error`].
    pub fn new<E>(kind: ErrorKind, error: E) -> Self
    where
        E: Into<Box<dyn std::error::Error + Send + Sync>>,
    {
        Self {
            repr: Repr::Custom(Custom {
                kind,
                error: error.into(),
            }),
        }
    }

    /// The [`ErrorKind`] of this `Error`.
    pub fn kind(&self) -> &ErrorKind {
        match &self.repr {
            Repr::Simple(kind)
            | Repr::SimpleMessage(kind, ..)
            | Repr::Custom(Custom { kind, .. }) => kind,
        }
    }

    /// The message provided when this `Error` was constructed, or `None`.
    pub fn message(&self) -> Option<&str> {
        match &self.repr {
            Repr::SimpleMessage(_, message) => Some(message.borrow()),
            _ => None,
        }
    }

    #[must_use]
    pub fn with_message<C>(kind: ErrorKind, message: C) -> Self
    where
        C: Into<Cow<'static, str>>,
    {
        Self {
            repr: Repr::SimpleMessage(kind, message.into()),
        }
    }

    #[must_use]
    pub fn with_message_fn<F, C>(kind: ErrorKind, message: F) -> Self
    where
        Self: Sized,
        F: FnOnce() -> C,
        C: Into<Cow<'static, str>>,
    {
        Self {
            repr: Repr::SimpleMessage(kind, message().into()),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.repr {
            Repr::Simple(kind) => write!(f, "{kind}"),
            Repr::SimpleMessage(_, message) => write!(f, "{message}"),
            Repr::Custom(Custom { error, .. }) => write!(f, "{error}"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.repr {
            Repr::Custom(Custom { error, .. }) => Some(&**error),
            _ => None,
        }
    }
}

impl From<ErrorKind> for Error {
    fn from(kind: ErrorKind) -> Self {
        Self {
            repr: Repr::Simple(kind),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Self::new(ErrorKind::Io, error)
    }
}

impl From<u32> for Error {
    fn from(error: u32) -> Self {
        Self {
            repr: Repr::Simple(ErrorKind::Os(error as i32)),
        }
    }
}

impl From<FromUtf8Error> for Error {
    fn from(error: FromUtf8Error) -> Self {
        Self::new(ErrorKind::InvalidData, error)
    }
}

#[derive(Debug)]
enum Repr {
    Simple(ErrorKind),
    SimpleMessage(ErrorKind, Cow<'static, str>),
    Custom(Custom),
}

#[derive(Debug)]
struct Custom {
    kind: ErrorKind,
    error: Box<dyn std::error::Error + Send + Sync>,
}
