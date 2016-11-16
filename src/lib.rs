//! A lisp interpreter written in Rust.

#![feature(conservative_impl_trait)]

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


extern crate lalrpop_util;
extern crate dot;

pub mod ast;
mod errors;
pub mod helpers;

#[allow(clippy, missing_docs, dead_code)]
mod grammar;

// Re-exports for convenience
pub use ast::parse;
// pub use errors::{LishpError, LishpResult};
