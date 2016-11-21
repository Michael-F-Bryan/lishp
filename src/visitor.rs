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
/// Note that all `visit_*()` methods take in a mutable reference to a `Type`.
/// This means that a visitor can change literally **anything** in the AST if
/// they want to, making it easy for you to add in arbitrary optimiser passes
/// or "compiler plugins" before the actual code is run.
///
/// # Examples
/// ```
/// # use lishp::types::Type;
/// use lishp::visitor::Visitor;
/// struct MyVisitor;
///
/// // simply writing `impl Visitor for MyVisitor {}` would give us access to
/// // all the `Visitor` visit_*() functions. Therefore, you only need to
/// // override the methods you need.
/// impl Visitor for MyVisitor {
///   fn visit_symbol(&mut self, s: &mut Type) {
///     println!("Visiting symbol: {:?}", s);
///   }
/// }
/// ```
#[allow(unused_variables)]
pub trait Visitor {
    /// The default behaviour is to delegate to either `visit_list()` or
    /// `visit_atom()` depending on what type of AST node it is.
    fn visit(&mut self, node: &mut Type) {
        match *node {
            Type::List(_) => self.visit_list(node),
            _ => self.visit_atom(node),
        }
    }

    /// Just recursively visit each node in the list.
    fn visit_list(&mut self, node: &mut Type) {
        match *node {
            Type::List(ref mut list) => {
                for node in list.iter_mut() {
                    self.visit(node);
                }
            }
            _ => unreachable!("Should never get anything other than a List in visit_list()"),
        }
    }

    /// Visiting an atom simply delegates to the appropriate visitor for that
    /// node type (`visit_boolean()`, `visit_integer()`, etc).
    fn visit_atom(&mut self, node: &mut Type) {
        match *node {
            Type::Boolean(_) => self.visit_boolean(node),
            Type::Integer(_) => self.visit_integer(node),
            Type::Float(_) => self.visit_float(node),
            Type::String(_) => self.visit_string(node),
            Type::Symbol(_) => self.visit_symbol(node),
            Type::Nil => {
                // this should be a no-op
            }
            _ => unreachable!("Shouldn't have any Lists here"),
        }
    }

    /// Visit a boolean. Default behaviour is a no-op.
    fn visit_boolean(&mut self, b: &mut Type) {}

    /// Visit an integer. Default behaviour is a no-op.
    fn visit_integer(&mut self, i: &mut Type) {}

    /// Visit a float. Default behaviour is a no-op.
    fn visit_float(&mut self, f: &mut Type) {}

    /// Visit a string. Default behaviour is a no-op.
    fn visit_string(&mut self, s: &mut Type) {}

    /// Visit a symbol. Default behaviour is a no-op.
    fn visit_symbol(&mut self, s: &mut Type) {}
}


#[cfg(test)]
mod tests {
    use super::*;
    use types::Type;

    struct DummyVisitor {
        visit_count: usize,
    }

    impl Visitor for DummyVisitor {
        fn visit_boolean(&mut self, _: &mut Type) {
            self.visit_count += 1;
        }

        fn visit_integer(&mut self, _: &mut Type) {
            self.visit_count += 1;
        }

        fn visit_float(&mut self, _: &mut Type) {
            self.visit_count += 1;
        }

        fn visit_string(&mut self, _: &mut Type) {
            self.visit_count += 1;
        }

        fn visit_symbol(&mut self, _: &mut Type) {
            self.visit_count += 1;
        }
    }

    #[test]
    fn visit_all_atoms() {
        let inputs =
            vec![t!(Bool, false), t!(Int, 5), t!(Float, 3.14), t!(String, "foo"), t!(Sym, "foo")];

        for mut input in inputs {
            let mut visitor = DummyVisitor { visit_count: 0 };

            visitor.visit(&mut input);

            assert_eq!(visitor.visit_count, 1);
        }
    }

    #[test]
    fn visit_a_list() {
        let mut ast = t!(List,
                         [t!(Bool, false),
                          t!(Int, 5),
                          t!(Float, 3.14),
                          t!(String, "foo"),
                          t!(Sym, "foo"),
                          t!(Nil)]);
        let mut visitor = DummyVisitor { visit_count: 0 };

        visitor.visit(&mut ast);

        assert_eq!(visitor.visit_count, 5);
    }

}
