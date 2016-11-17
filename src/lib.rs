//! A lisp interpreter written in Rust.
//!
//! The interpreter itself can be used as a library, which means it's possible
//! to give your applications a scripting engine.
//!
//! # Examples
//!

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
    unused_results,
    variant_size_differences)]

extern crate regex;

// Sub-modules of the lishp crate
pub mod lexer;
