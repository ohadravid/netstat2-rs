use std;

/// General error type.
#[derive(Clone, Debug)]
pub enum Error {
    /// Error originating from Rust code.
    InternalError(&'static str),
    /// Error originating from FFI calls.
    ForeignError {
        api_name: &'static str,
        err_code: i32,
    },
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::InternalError(msg) => write!(f, "InternalError: {}", msg),
            Error::ForeignError { api_name, err_code } => write!(
                f,
                "ForeignError with code {} occured in `{}`.",
                err_code, api_name
            ),
        }
    }
}

impl std::error::Error for Error {}
