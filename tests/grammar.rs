extern crate lishp;
use lishp::{parse_atom, parse_sexpr};
use lishp::ast::{Atom, Sexpr, List};


#[test]
fn parse_nested_sexpr() {
    let some_list = List::with(vec![Sexpr::atom(Atom::Int(1)),
                                    Sexpr::list(List::with(vec![
                                        Sexpr::atom(Atom::Int(2)),
                                        Sexpr::atom(Atom::Int(3)),
                                        ]))]);
    let src = "(1 (2 3))";
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
    let inputs = vec![
            // Integers
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
            ("_foo_", Atom::Symbol("_foo_".to_string())),

            ];

    for (src, should_be) in inputs {
        let got = parse_atom(src);
        assert_eq!(got, Ok(should_be));
    }
}
