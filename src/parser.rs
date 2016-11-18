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
        unimplemented!()
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
            // first check to see if it's a bool
            if let Ok(b) = next_token.parse::<bool>() {
                Ok(Type::Boolean(b))
            } else {
                Ok(Type::Symbol(next_token.value().to_string()))
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use lexer;
    use super::*;
    use types::Type;

    #[test]
    #[ignore]
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
                          (tok!("foo"), Type::Symbol("foo".to_string())),
                          (tok!("\"foo\""), Type::String("foo".to_string()))];

        for (src, should_be) in inputs {
            let mut parser = Parser::new(vec![src.clone()]);
            let got = parser.parse();
            println!("src: {:?}, should be: {:?}, got: {:?}", src, should_be, got);
            assert_eq!(got, Ok(should_be));
        }
    }
}
