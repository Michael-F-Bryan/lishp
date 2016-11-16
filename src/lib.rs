//! A lisp interpreter written in Rust.
//!
//! The interpreter itself can be used as a library, which means it's possible
//! to give your applications a scripting engine.
//!
//! # Examples
//!
//! include!("src/bin/main.rs");
//!
//! ```
//! extern crate lishp;
//!
//! let src = "(print 1 2)";
//! let ast = lishp::parse(src).unwrap();
//! ```

// TODO: Update example when code evaluation works

#![deny(missing_docs,
        missing_debug_implementations, missing_copy_implementations,
        trivial_casts,
        unsafe_code,
        unused_import_braces, unused_qualifications)]

// So compiling without clippy doesn't print a warning
#![allow(unknown_lints)]

// Add some clippy-specific stuff
#![cfg_attr(feature = "dev", allow(unstable_features))]
#![cfg_attr(feature = "dev", feature(plugin))]
#![cfg_attr(feature = "dev", plugin(clippy))]

// External dependencies
extern crate lalrpop_util;
extern crate dot;

// Sub-modules of the lishp crate
#[allow(clippy, missing_docs, dead_code)]
pub mod ast;
pub mod helpers;
mod grammar;
mod errors;

// Re-exports for convenience

pub use ast::parse;
// pub use errors::{LishpError, LishpResult};
