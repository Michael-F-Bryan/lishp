//! This module contains the underlying types that the AST consists of.


use grammar;
use lalrpop_util;
use std::collections::VecDeque;
use std::fmt::{Formatter, Display, Result as FmtResult};
use std::iter::IntoIterator;


/// Parse source code into its abstract syntax tree representation.
pub fn parse(src: &str) -> Result<Sexpr, lalrpop_util::ParseError<usize, (usize, &str), ()>> {
    grammar::parse_sexpr(src)
}


/// Atoms are the most fundamental elements in the language. They consist of
/// things like integers, floats, strings, bools, and symbols.
#[derive(Debug, PartialEq)]
pub enum Atom {
    /// A 64 bit floating point number.
    Float(f64),

    /// A unicode string.
    String(String),

    /// A 64 bit signed integer.
    Int(i64),

    /// A boolean value.
    Bool(bool),

    /// A symbol. Usually corresponds to the name of a function, variable, or
    /// other object.
    Symbol(String),
}

/// A Sexpr is either a single Atom, or a list of more Sexpr's.
#[derive(Debug, PartialEq)]
#[allow(non_camel_case_types)]
pub enum Sexpr {
    /// A wrapper around an Atom.
    atom(Atom),

    /// A wrapper around a List.
    list(List),
}

/// A list of Sexpr's.
#[derive(Debug, Default, PartialEq)]
pub struct List {
    elements: VecDeque<Box<Sexpr>>,
}

impl List {
    /// Create a new empty List.
    pub fn new() -> Self {
        List::default()
    }

    /// Add an element to the end of the list.
    pub fn push(&mut self, s: Sexpr) {
        self.elements.push_back(Box::new(s));
    }

    /// Create a list of Sexpr's out of any iterable.
    pub fn with<T: IntoIterator<Item = Sexpr>>(items: T) -> List {
        let mut l = List::new();
        for item in items {
            l.push(item);
        }
        l
    }

    /// Get an element from the list.
    pub fn get(&self, index: usize) -> Option<&Box<Sexpr>> {
        self.elements.get(index)
    }

    /// Get the number of items in this List.
    pub fn len(&self) -> usize {
        self.elements.len()
    }

    /// Get an iterator over the elements in the list.
    pub fn iter<'a>(&'a self) -> ::std::collections::vec_deque::Iter<Box<Sexpr>> {
        self.elements.iter()
    }
}

impl IntoIterator for List {
    type Item = Box<Sexpr>;
    type IntoIter = ::std::collections::vec_deque::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.elements.into_iter()
    }
}

impl Display for Atom {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match *self {
            Atom::Float(ref n) => write!(f, "{}", n),
            Atom::Int(ref n) => write!(f, "{}", n),
            Atom::String(ref s) => write!(f, "{}", s),
            Atom::Bool(ref b) => write!(f, "{}", if *b { "true" } else { "false" }),
            Atom::Symbol(ref s) => write!(f, "{}", s),
        }
    }
}

impl Display for List {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        let mut repr = String::new();
        repr.push_str("(");

        let args = self.elements.iter().map(|e| format!("{}", e)).collect::<Vec<_>>().join(" ");
        repr.push_str(&args);
        repr.push_str(")");
        write!(f, "{}", repr)
    }
}

impl Display for Sexpr {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match *self {
            Sexpr::atom(ref a) => write!(f, "{}", a),
            Sexpr::list(ref l) => write!(f, "{}", l),
        }
    }
}


#[cfg(test)]
mod tests {
    use grammar::{parse_atom, parse_sexpr};
    use super::*;


    #[test]
    fn check_atoms_display_correctly() {
        let inputs = vec![(Atom::String("a".to_string()), "a")];

        for (elem, should_be) in inputs {
            let got = format!("{}", elem);
            assert_eq!(got, should_be);
        }
    }

    #[test]
    fn parse_nested_sexpr() {
        let src = "(1 (2 3))";
        let some_list = List::with(vec![Sexpr::atom(Atom::Int(1)),
                                        Sexpr::list(List::with(vec![Sexpr::atom(Atom::Int(2)),
                                                                    Sexpr::atom(Atom::Int(3))]))]);
        let should_be = Sexpr::list(some_list);
        let got = parse_sexpr(src);
        assert_eq!(got, Ok(should_be));
    }

    #[test]
    fn parse_empty_sexpr() {
        let src = "()";
        let should_be = Sexpr::list(List::new());
        let got = parse_sexpr(src);
        assert_eq!(got, Ok(should_be));
    }

    #[test]
    fn parse_basic_sexpr() {
        let some_list = List::with(vec![Sexpr::atom(Atom::Int(1)),
                                        Sexpr::atom(Atom::Int(2)),
                                        Sexpr::atom(Atom::Int(3))]);
        let src = "( 1 2 3)";
        let should_be = Sexpr::list(some_list);
        let got = parse_sexpr(src);
        assert_eq!(got, Ok(should_be));
    }

    #[test]
    fn parse_all_atoms() {
        let inputs = vec![// Integers
                          ("123", Atom::Int(123)),
                          ("-123", Atom::Int(-123)),

                          // Floats
                          ("12.3", Atom::Float(12.3)),
                          ("-12.3", Atom::Float(-12.3)),

                          // Strings
                          (r#""foo""#, Atom::String("foo".to_string())),
                          (r#""""#, Atom::String("".to_string())),

                          // Bools
                          ("true", Atom::Bool(true)),
                          ("false", Atom::Bool(false)),

                          // Symbols
                          ("if", Atom::Symbol("if".to_string())),
                          ("Foo", Atom::Symbol("Foo".to_string())),
                          ("a3", Atom::Symbol("a3".to_string())),
                          ("equal?", Atom::Symbol("equal?".to_string())),
                          ("equal!", Atom::Symbol("equal!".to_string())),
                          ("!", Atom::Symbol("!".to_string())),
                          ("*", Atom::Symbol("*".to_string())),
                          ("/", Atom::Symbol("/".to_string())),
                          ("-", Atom::Symbol("-".to_string())),
                          ("+", Atom::Symbol("+".to_string())),
                          ("%", Atom::Symbol("%".to_string())),
                          ("_", Atom::Symbol("_".to_string())),
                          ("_foo_", Atom::Symbol("_foo_".to_string()))];

        for (src, should_be) in inputs {
            let got = parse_atom(src);
            assert_eq!(got, Ok(should_be));
        }
    }

}
