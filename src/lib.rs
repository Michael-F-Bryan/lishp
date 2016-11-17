//! A lisp interpreter written in Rust.
//!
//! The interpreter itself can be used as a library, which means it's possible
//! to give your applications a scripting engine.
//!
//! # Examples
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


// Sub-modules of the lishp crate
mod lexer;
