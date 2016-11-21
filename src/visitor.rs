//! Whenever you want to do something with the AST, you generally need to visit
//! each node of the tree. To make this job easier, there's the `Visitor` trait.
//!
//! Later on when the language gains Objects and Classes, implementing your own
//! `Visitor` will give it access to much more information. In particular, all
//! the methods associated with the Object, any docstrings, etc.

use types::Type;

/// The generic `Visitor` trait. Anything implementing this trait will be able
/// to visit the nodes in an AST.
///
/// # Examples
/// ```
/// use lishp::visitor::Visitor;
/// struct MyVisitor;
///
/// // simply writing `impl Visitor for MyVisitor {}` would give us access to
/// // all the `Visitor` visit_*() functions. Therefore, you only need to
/// // override the methods you need.
/// impl Visitor for MyVisitor {
///   fn visit_symbol(&mut self, s: &String) {
///     println!("Visiting symbol: {}", s);
///   }
/// }
/// ```
#[allow(unused_variables)]
pub trait Visitor {
    /// The default behaviour is to delegate to either `visit_list()` or
    /// `visit_atom()` depending on what type of AST node it is.
    fn visit(&mut self, node: &Type) {
        match *node {
            Type::List(ref l) => self.visit_list(l),
            ref atom => self.visit_atom(atom),
        }
    }

    /// Just recursively visit each node in the list.
    fn visit_list(&mut self, nodes: &Vec<Type>) {
        for node in nodes {
            self.visit(node);
        }
    }

    /// Visiting an atom simply delegates to the appropriate visitor for that
    /// node type (`visit_boolean()`, `visit_integer()`, etc).
    fn visit_atom(&mut self, node: &Type) {
        match *node {
            Type::Boolean(ref b) => self.visit_boolean(b),
            Type::Integer(ref i) => self.visit_integer(i),
            Type::Float(ref f) => self.visit_float(f),
            Type::String(ref s) => self.visit_string(s),
            Type::Symbol(ref s) => self.visit_symbol(s),
            Type::Nil => {
                // this should be a no-op
            }
            _ => unreachable!("Shouldn't have any Lists here"),
        }
    }

    /// Visit a boolean. Default behaviour is a no-op.
    fn visit_boolean(&mut self, b: &bool) {}

    /// Visit an integer. Default behaviour is a no-op.
    fn visit_integer(&mut self, i: &i64) {}

    /// Visit a float. Default behaviour is a no-op.
    fn visit_float(&mut self, f: &f64) {}

    /// Visit a string. Default behaviour is a no-op.
    fn visit_string(&mut self, s: &String) {}

    /// Visit a symbol. Default behaviour is a no-op.
    fn visit_symbol(&mut self, s: &String) {}
}


#[cfg(test)]
mod tests {
    use super::*;

    struct DummyVisitor {
        visit_count: usize,
    }

    impl Visitor for DummyVisitor {
        fn visit_boolean(&mut self, _: &bool) {
            self.visit_count += 1;
        }

        fn visit_integer(&mut self, _: &i64) {
            self.visit_count += 1;
        }

        fn visit_float(&mut self, _: &f64) {
            self.visit_count += 1;
        }

        fn visit_string(&mut self, _: &String) {
            self.visit_count += 1;
        }

        fn visit_symbol(&mut self, _: &String) {
            self.visit_count += 1;
        }
    }

    #[test]
    fn visit_all_atoms() {
        let inputs =
            vec![t!(Bool, false), t!(Int, 5), t!(Float, 3.14), t!(String, "foo"), t!(Sym, "foo")];

        for input in inputs {
            let mut visitor = DummyVisitor { visit_count: 0 };

            visitor.visit(&input);

            assert_eq!(visitor.visit_count, 1);
        }
    }

    #[test]
    fn visit_a_list() {
        let ast = t!(List,
                     [t!(Bool, false),
                      t!(Int, 5),
                      t!(Float, 3.14),
                      t!(String, "foo"),
                      t!(Sym, "foo"),
                      t!(Nil)]);
        let mut visitor = DummyVisitor { visit_count: 0 };

        visitor.visit(&ast);

        assert_eq!(visitor.visit_count, 5);
    }

}
