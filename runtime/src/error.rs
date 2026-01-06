#[allow(dead_code)]
#[derive(Debug)]
pub enum Error {
    StateAlreadyInitializedError,
    StateNotInitializedError,
    PoisonError(String),
    IOError(String),
    SystemTimeError(String),
    ParseError(String),
    LogicError(String),
    ChannelSendError(String),
    ChannelReceiveError(String),
    WatcherError(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::StateAlreadyInitializedError => {
                write!(f, "1|Static state is ALREADY initialized")
            }
            Error::StateNotInitializedError => write!(f, "2|Static state is NOT initialized"),
            Error::PoisonError(err_str) => write!(f, "3|{}", err_str),
            Error::IOError(err_str) => write!(f, "4|{}", err_str),
            Error::SystemTimeError(err_str) => write!(f, "5|{}", err_str),
            Error::ParseError(err_str) => write!(f, "6|{}", err_str),
            Error::LogicError(err_str) => write!(f, "7|{}", err_str),
            Error::ChannelSendError(err_str) => write!(f, "8|{}", err_str),
            Error::ChannelReceiveError(err_str) => write!(f, "9|{}", err_str),
            Error::WatcherError(err_str) => write!(f, "10|{}", err_str),
        }
    }
}

impl<T> From<std::sync::PoisonError<T>> for Error {
    fn from(value: std::sync::PoisonError<T>) -> Self {
        Error::PoisonError(value.to_string())
    }
}
impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Error::IOError(value.to_string())
    }
}
impl From<std::time::SystemTimeError> for Error {
    fn from(value: std::time::SystemTimeError) -> Self {
        Error::SystemTimeError(value.to_string())
    }
}
