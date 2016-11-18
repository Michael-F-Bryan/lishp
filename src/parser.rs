//! A parser for turning a list of Tokens into an Abstract Syntax Tree.

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
}

impl Parser {
    /// Create a new Parser using a list of Tokens.
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser {
            tokens: tokens,
            position: 0,
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
        self.parse_form()
    }

    fn parse_form(&mut self) -> LishpResult<Type> {
        if self.tokens.len() == 0 {
            return Ok(Type::Nil);
        }

        if self.peek().unwrap().starts_with("(") {
            self.parse_list()
        } else {
            self.parse_atom()
        }
    }

    fn parse_list(&mut self) -> LishpResult<Type> {
        let mut components: Vec<Type> = Vec::new();
        if cfg!(test) {
            println!("Next token: {:?}", self.peek());
        }

        // consume the open paren
        let _ = self.next();

        // otherwise keep parsing atoms until you hit that closing paren
        while !self.peek().ok_or(LishpError::EOF)?.starts_with(")") {
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
        let next_token = self.next().ok_or(LishpError::EOF)?;
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
            Ok(Type::String(letters.into_iter().collect()))
        } else {
            match next_token.value() {
                "nil" => Ok(Type::Nil),
                "true" => Ok(Type::Boolean(true)),
                "false" => Ok(Type::Boolean(false)),
                other => Ok(Type::Symbol(other.to_string())),
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use types::Type;

    #[test]
    fn parse_nil_expressions() {
        let inputs = vec![vec![], vec![tok!("("), tok!(")")], vec![tok!("nil")]];

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
        let inputs = vec![(toks!("(", "foo", ")"), t!(List, [t!(Sym, "foo")]))];

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
}
