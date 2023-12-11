pub type Result<T, E = Error> = std::result::Result<T, E>;

pub enum ErrorKind {
    InvalidPath,
    InvalidArg,
}

impl ToString for ErrorKind {
    fn to_string(&self) -> String {
        match self {
            Self::InvalidPath => "The provided path(s) contain no valid images.".to_string(),
            Self::InvalidArg => "Failed to parse (ARG or whitelisted.txt) into u64.".to_string(),
        }
    }
}

impl From<std::num::ParseIntError> for ErrorKind {
    fn from(_: std::num::ParseIntError) -> Self {
        Self::InvalidArg
    }
}

pub enum Error {
    IO(std::io::Error),
    Env(std::env::VarError),
    Http(serenity::Error),
    Misc(ErrorKind),
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self::IO(value)
    }
}

impl From<std::env::VarError> for Error {
    fn from(value: std::env::VarError) -> Self {
        Self::Env(value)
    }
}

impl From<serenity::Error> for Error {
    fn from(value: serenity::Error) -> Self {
        Self::Http(value)
    }
}

impl<T: Into<ErrorKind>> From<T> for Error {
    fn from(value: T) -> Self {
        Self::Misc(value.into())
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&match self {
            Self::IO(e) => e.to_string(),
            Self::Env(e) => e.to_string(),
            Self::Http(e) => e.to_string(),
            Self::Misc(e) => e.to_string(),
        })
    }
}

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self, f)
    }
}

impl std::error::Error for Error {}
