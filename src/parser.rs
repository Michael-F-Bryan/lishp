//! A parser for turning a list of Tokens into an Abstract Syntax Tree.

// TODO: add proper error handling for unbalanced parens

use errors::{LishpError, LishpResult};
use lexer::Token;
use types::Type;


/// The Parser.
///
/// # Examples
#[derive(Debug)]
pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
    parens_stack: Vec<usize>,
}

impl Parser {
    /// Create a new Parser using a list of Tokens.
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser {
            tokens: tokens,
            position: 0,
            parens_stack: vec![],
        }
    }

    /// Look at the next Token, but don't consume it.
    pub fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.position)
    }

    /// Consume the next token in the stream.
    pub fn next(&mut self) -> Option<&Token> {
        let tok = self.tokens.get(self.position);
        if tok.is_some() {
            self.position += 1;
        }
        tok
    }

    /// Do the actual parsing and get the resultant AST.
    pub fn parse(&mut self) -> LishpResult<Type> {
        let ast = self.parse_form()?;
        if self.position != self.tokens.len() {
            Err(self.eof())
        } else {
            Ok(ast)
        }
    }

    fn parse_form(&mut self) -> LishpResult<Type> {
        if self.tokens.len() == 0 {
            return Ok(Type::Nil);
        }

        // try to consume a '(', if we can then we need to parse a list
        if let Some(_) = self.chomp_open_paren() {
            self.parse_list()
        } else {
            self.parse_atom()
        }
    }

    fn parse_list(&mut self) -> LishpResult<Type> {
        let mut components: Vec<Type> = Vec::new();

        // otherwise keep parsing atoms until you hit that closing paren
        while let None = self.chomp_close_paren() {
            let next_atom = self.parse_form()?;
            components.push(next_atom);
        }

        if components.len() == 0 {
            Ok(Type::Nil)
        } else {
            Ok(Type::List(components))
        }
    }

    fn parse_atom(&mut self) -> LishpResult<Type> {
        if self.position >= self.tokens.len() {
            return Err(self.eof());
        }

        let next_token = self.next().unwrap();

        if next_token.starts_with_number() {
            // try parsing the token as a number
            if let Ok(int) = next_token.parse::<i64>() {
                Ok(Type::Integer(int))
            } else {
                let float: f64 = next_token.parse()?;
                Ok(Type::Float(float))
            }
        } else if next_token.starts_with("\"") {
            let mut letters: Vec<char> = next_token.value().chars().collect();
            debug_assert!(letters.len() >= 2);
            let _ = letters.pop();  // get rid of the trailing quote
            let _ = letters.remove(0);

            // Collect the characters back into a string and do the usual
            // escapes (\n, \t, etc)
            let no_quotes =
                letters.into_iter().collect::<String>().replace(r"\n", "\n").replace(r"\t", "\t");

            Ok(Type::String(no_quotes))
        } else {
            match next_token.value() {
                "nil" => Ok(Type::Nil),
                "true" => Ok(Type::Boolean(true)),
                "false" => Ok(Type::Boolean(false)),
                other => Ok(Type::Symbol(other.to_string())),
            }
        }
    }

    fn chomp_open_paren(&mut self) -> Option<&Token> {
        if let Some(is_paren) = self.peek().map(|tok| tok == "(") {
            if is_paren {
                self.parens_stack.push(self.position);
                return self.next();
            }
        }
        None
    }

    fn chomp_close_paren(&mut self) -> Option<&Token> {
        if let Some(is_paren) = self.peek().map(|tok| tok == ")") {
            if is_paren {
                let _ = self.parens_stack.pop();
                return self.next();
            }
        }
        None
    }

    fn eof(&self) -> LishpError {
        LishpError::EOF(*self.parens_stack.get(0).unwrap_or(&0))
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use types::Type;

    #[test]
    fn parse_nil_expressions() {
        let inputs = vec![vec![], toks!("(", ")"), toks!("nil")];

        for input in inputs {
            let mut parser = Parser::new(input.clone());
            let got = parser.parse();
            println!("src: {:?}, should be: {:?}, got: {:?}",
                     input,
                     Type::Nil,
                     got);
            assert_eq!(got, Ok(Type::Nil));
        }
    }

    #[test]
    fn parse_valid_atoms() {
        let inputs = vec![(tok!("1"), Type::Integer(1)),
                          (tok!("1.23"), Type::Float(1.23)),
                          (tok!("true"), Type::Boolean(true)),
                          (tok!("false"), Type::Boolean(false)),
                          (tok!("nil"), Type::Nil),
                          (tok!("foo"), t!(Sym, "foo")),
                          (tok!("\"foo\""), t!(String, "foo"))];

        for (src, should_be) in inputs {
            let mut parser = Parser::new(vec![src.clone()]);
            let got = parser.parse();
            println!("src: {:?}, should be: {:?}, got: {:?}", src, should_be, got);
            assert_eq!(got, Ok(should_be));
        }
    }

    #[test]
    fn parse_basic_lists() {
        let inputs = vec![(toks!("(", "foo", ")"), t!(List, [t!(Sym, "foo")])),
                          (toks!("(", "+", "1.23", ")"),
                           t!(List, [t!(Sym, "+"), t!(Float, 1.23)]))];

        for (src, should_be) in inputs {
            let mut parser = Parser::new(src.clone());
            let got = parser.parse();
            println!("src: {:?}, should be: {:?}, got: {:?}",
                     src.iter().map(|i| i.value()).collect::<Vec<_>>(),
                     should_be,
                     got);
            assert_eq!(got, Ok(should_be));
        }
    }

    #[test]
    fn simple_nested_list() {
        let tokens = toks!("(", "foo", "(", "9", ")", ")");
        let should_be = t!(List, [t!(Sym, "foo"), t!(List, [t!(Int, 9)])]);

        let mut parser = Parser::new(tokens.clone());
        let got = parser.parse();
        println!("tokens: {:?}, should be: {:?}, got: {:?}",
                 tokens.iter().map(|i| i.value()).collect::<Vec<_>>(),
                 should_be,
                 got);
        assert_eq!(got, Ok(should_be));
    }

    #[test]
    fn nested_list_with_list_in_first_position() {
        let tokens = toks!("(", "(", "*", ")", "(", "32", ")", "true", "1.23", ")");
        let should_be = t!(List,
                           [t!(List, [t!(Sym, "*")]),
                            t!(List, [t!(Int, 32)]),
                            t!(Bool, true),
                            t!(Float, 1.23)]);

        let mut parser = Parser::new(tokens.clone());
        let got = parser.parse();
        println!("tokens: {:?}, should be: {:?}, got: {:?}",
                 tokens.iter().map(|i| i.value()).collect::<Vec<_>>(),
                 should_be,
                 got);
        assert_eq!(got, Ok(should_be));
    }

    #[test]
    fn string_escapes_are_done_correctly() {
        let inputs = vec![(r#""foo\n""#, "foo\n"), (r#""foo\t""#, "foo\t")];

        for (from, to) in inputs {
            let tok = tok!(from);
            let mut parser = Parser::new(vec![tok]);
            if let Ok(Type::String(s)) = parser.parse() {
                assert_eq!(s, to);
            } else {
                unreachable!();
            }

        }
    }

    #[test]
    fn unbalanced_parens() {
        let inputs = vec![toks!("(", "foo"),
                          toks!("asd", ")"),
                          toks!("(", "foo", "(", "123", ")"),
                          toks!("(", "foo", "(", "123", ")", "(", ")")];

        for tokens in inputs {
            let mut parser = Parser::new(tokens.clone());
            let got = parser.parse();
            assert!(got.is_err());
            println!("src: {:?}, got: {:?}",
                     tokens.iter().map(|t| t.value()).collect::<String>(),
                     got);
        }
    }
}
