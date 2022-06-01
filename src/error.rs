use std::fmt;

use nom::error::Error;
use nom::Err;

#[derive(Clone, Debug)]
pub struct ApplicationError {
    message: String,
    kind: ApplicationErrorKind,
}

#[derive(Clone, Copy, Debug)]
pub enum ApplicationErrorKind {
    Input,
    Io,
}

impl ApplicationError {
    fn new(message: impl AsRef<str>, kind: ApplicationErrorKind) -> Self {
        Self {
            message: message.as_ref().to_string(),
            kind,
        }
    }

    pub fn input(explanation: impl AsRef<str>) -> Self {
        Self::new(explanation, ApplicationErrorKind::Input)
    }

    pub fn io(explanation: impl AsRef<str>) -> Self {
        Self::new(explanation, ApplicationErrorKind::Io)
    }
}

impl fmt::Display for ApplicationErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let string = match self {
            ApplicationErrorKind::Input => "input",
            ApplicationErrorKind::Io => "io",
        };
        write!(f, "{}", string)
    }
}

impl fmt::Display for ApplicationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}; {}", self.kind, self.message)
    }
}

impl From<nom::Err<nom::error::Error<&str>>> for ApplicationError {
    fn from(error: Err<Error<&str>>) -> Self {
        ApplicationError::input(format!("Unable to parse input: {}", error))
    }
}

impl From<std::io::Error> for ApplicationError {
    fn from(err: std::io::Error) -> Self {
        ApplicationError::io(format!("{}", err))
    }
}
