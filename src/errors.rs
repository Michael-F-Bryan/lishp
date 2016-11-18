//! The common error types used in `Lishp`.

use std::convert::From;
use std::fmt::{self, Display, Formatter};
use std::num::ParseFloatError;


/// A shortcut for any Result which contains a LishpError.
pub type LishpResult<T> = Result<T, LishpError>;


/// All the errors specific to Lishp.
#[derive(Debug, PartialEq)]
pub enum LishpError {
    /// End of file reached prematurely. The parser will tell you where it
    /// thinks you fucked up.
    EOF(usize),

    /// Converting the token to a number was unsuccessful.
    InvalidNumber(ParseFloatError),

    /// There aren't a balanced number of parentheses. The parser tries to
    /// figure out which parentheses you forgot to close.
    UnbalancedParens(usize),
}

impl Display for LishpError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            LishpError::EOF(_) => write!(f, "Reached end of file before parsing finished"),
            LishpError::InvalidNumber(ref e) => write!(f, "InvalidNumber: {}", e),
            LishpError::UnbalancedParens(_) => write!(f, "Unbalanced parentheses"),
        }
    }
}

impl From<ParseFloatError> for LishpError {
    fn from(other: ParseFloatError) -> Self {
        LishpError::InvalidNumber(other)
    }
}
