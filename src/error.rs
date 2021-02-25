#[derive(Debug)]
pub enum Error {
    User(ErrorKind),
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ErrorKind {
    Unauthorized,
    InvalidToken,
    NotCreate,
    NotSave,
}

impl ErrorKind {
    fn as_str(&self) -> &str {
        match *self {
            ErrorKind::Unauthorized => "unauthorized.",
            ErrorKind::InvalidToken => "this token is imvalid",
            ErrorKind::NotCreate => "problem to create user",
            ErrorKind::NotSave => "problem to save user",
        }
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::User(ref err) => err.as_str(),
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Error::User(ref err) => write!(f, "User error: {:?}", err),
        }
    }
}
