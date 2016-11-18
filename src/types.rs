//! The base data-type used in Lishp.

/// An enum containing all the possible data types that can be used in Lishp.
#[derive(Debug, PartialEq)]
pub enum Type {
    /// A list containing other Types.
    List(Vec<Type>),

    /// A 64 bit signed integer.
    Integer(i64),

    /// A 64 bit floating point number.
    Float(f64),

    /// A basic utf-8 string.
    String(String),

    /// A symbol.
    Symbol(String),

    /// A boolean value.
    Boolean(bool),

    /// Nothing...
    Nil,
}
