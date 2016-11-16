
// This is a work in progress, so explicitly ignore all lints for now
#![allow(missing_docs, unused_imports, dead_code)]

use std::convert::From;
use std::fmt::{Formatter, Display, Result as FmtResult};
use std::error::Error;

pub type LishpResult<T> = Result<T, LishpError>;

#[derive(Debug, PartialEq)]
pub enum LishpError {
    Foo,
}

impl Display for LishpError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match *self {
            LishpError::Foo => write!(f, "Foo"),
        }
    }
}
impl Error for LishpError {
    fn description(&self) -> &str {
        unimplemented!()
    }
}
