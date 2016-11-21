//! A lisp interpreter written in Rust.
//!
//! The interpreter itself can be used as a library, so as an added bonus, it's
//! fairly easy to use as an embedded scripting engine.
//!
//! # Examples
//!
//! This library was designed to be easy to use and from existing Rust code so
//! you have full access to all the internals. This makes tokenizing source
//! code a breeze.
//!
//! ```
//! let src = r#"(print "5 + (9 % 2) = " (+ 5 (% 9 2)))"#;
//! let tokens = lishp::tokenize(src).unwrap();
//! ```
//!
//! Generating an Abstract Syntax Tree isn't terribly difficult either.
//!
//! ```
//! # use lishp::parser::Parser;
//! # let src = "(print (+ 5 (% 9 2)))";
//! # let tokens = lishp::tokenize(src).unwrap();
//! let mut parser = Parser::new(tokens);
//! let ast = parser.parse().unwrap();
//! ```

// TODO: Update example when code evaluation works

// Some extra lints
#![deny(missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unstable_features,
    unused_import_braces,
    unused_qualifications,
    unused_extern_crates,
    unused_results)]

extern crate regex;

#[macro_use]
mod macros;

// Sub-modules of the lishp crate
pub mod lexer;
pub mod parser;
pub mod errors;
pub mod types;
pub mod visitor;

// re-export for convenience

pub use errors::{LishpResult, LishpError};
pub use lexer::tokenize;
pub use parser::Parser;
pub use types::Type;
