//! Various useful helper functions.


use ast::Sexpr;
use dot;
use std::io::{Result as IoResult, Write};
use std::iter;


/// Walk the AST then use it to generate a Dot graph and write it to a file-like
/// object.
pub fn render<W: Write>(ast: Sexpr, writer: &mut W) -> IoResult<usize> {
    unimplemented!()
}
