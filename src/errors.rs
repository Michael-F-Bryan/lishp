//! The common error types used in `Lishp`.

use std::convert::From;
use std::fmt::{self, Display, Formatter};
use std::num::ParseFloatError;


/// A shortcut for any Result which contains a LishpError.
pub type LishpResult<T> = Result<T, LishpError>;


/// All the errors specific to Lishp.
#[derive(Debug, PartialEq)]
pub enum LishpError {
    /// End of file reached prematurely.
    EOF,

    /// Converting the token to a number was unsuccessful.
    InvalidNumber(ParseFloatError),
}

impl Display for LishpError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            LishpError::EOF => write!(f, "EOF"),
            LishpError::InvalidNumber(ref e) => write!(f, "InvalidNumber: {}", e),
        }
    }
}

impl From<ParseFloatError> for LishpError {
    fn from(other: ParseFloatError) -> Self {
        LishpError::InvalidNumber(other)
    }
}
