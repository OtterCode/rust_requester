use std::sync::{Arc};

#[derive(Debug, Clone)]
pub enum Error {
    PortPermissionDenied,
    InvalidPort,
    AuthServerClosedEarly,
    PkceCodeVerifierLocked,
    PkceCodeVerifierMissing,
    KillSignalNotInitialized,
    MissingToken,
    Unknown(Arc<Box<dyn std::error::Error + Send + Sync>>),
}

impl From<Box<dyn std::error::Error + Send + Sync>> for Error {
    fn from(error: Box<dyn std::error::Error + Send + Sync>) -> Self {
        Error::Unknown(Arc::new(error))
    }
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        match error.kind() {
            std::io::ErrorKind::PermissionDenied => return Error::PortPermissionDenied,
            _ => error.into(),
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::PortPermissionDenied => {
                write!(f, "Cannot access low port without admin privileges.")
            }
            Error::AuthServerClosedEarly => {
                write!(f, "Auth server closed before receiving auth code.")
            }
            Error::PkceCodeVerifierLocked => write!(f, "Pkce code verifier is locked."),
            Error::PkceCodeVerifierMissing => write!(f, "Pkce code verifier is missing."),
            Error::InvalidPort => write!(f, "Invalid port number."),
            Error::KillSignalNotInitialized => write!(f, "Kill signal not initialized."),
            Error::MissingToken => write!(f, "Missing token."),
            Error::Unknown(error) => write!(f, "Unknown error: {:?}", error),
        }
    }
}

impl std::error::Error for Error {}
