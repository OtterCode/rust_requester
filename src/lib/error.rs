
#[derive(Debug)]
pub enum Error {
    PortPermissionDenied,
    AuthServerClosedEarly,
    Unknown(Box<dyn std::error::Error>),
}

impl From<Box<dyn std::error::Error>> for Error {
    fn from(error: Box<dyn std::error::Error>) -> Self {
        Error::Unknown(error)
    }
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        match error.kind() {
            std::io::ErrorKind::PermissionDenied => return Error::PortPermissionDenied,
            _ => Error::Unknown(Box::new(error))
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::PortPermissionDenied => write!(f, "Cannot access low port without admin privileges."),
            Error::AuthServerClosedEarly => write!(f, "Auth server closed before receiving auth code."),
            Error::Unknown(error) => write!(f, "Unknown error: {}", error),
        }
    }
}

impl std::error::Error for Error {}