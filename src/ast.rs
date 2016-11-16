use std::collections::VecDeque;


/// Atoms are the most fundamental elements in the language. They consist of
/// things like integers, floats, strings, bools, and symbols.
#[derive(Debug, PartialEq)]
pub enum Atom {
    Float(f64),
    String(String),
    Int(i64),
    Bool(bool),
    Symbol(String),
}

/// A Sexpr is either a single Atom, or a list of more Sexpr's.
#[derive(Debug, PartialEq)]
#[allow(non_camel_case_types)]
pub enum Sexpr {
    atom(Atom),
    list(List),
}

/// A list of Sexpr's.
#[derive(Debug, Default, PartialEq)]
pub struct List {
    elements: VecDeque<Box<Sexpr>>,
}

impl List {
    pub fn new() -> Self {
        List::default()
    }

    pub fn push(&mut self, s: Sexpr) {
        self.elements.push_back(Box::new(s));
    }

    pub fn with<T: IntoIterator<Item = Sexpr>>(items: T) -> List {
        let mut l = List::new();
        for item in items.into_iter() {
            l.push(item);
        }

        l
    }
}
