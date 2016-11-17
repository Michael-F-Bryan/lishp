//! This is the module containing the lexer.

use regex::Regex;
use std::str::FromStr;


/// The location of a Token in the source code. Start and end are the idices
/// that the token starts and ends at.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Span {
    start: usize,
    end: usize,
}

impl Span {
    /// Create a new span.
    pub fn new(start: usize, end: usize) -> Span {
        Span {
            start: start,
            end: end,
        }
    }
}

/// Small Error type used when an invalid token is encountered.
#[derive(PartialEq, Debug, Copy, Clone)]
pub struct InvalidTokenError {
    pos: usize,
}

/// A single token and its location in the source code.
#[derive(Clone, Debug, PartialEq)]
pub struct Token {
    value: String,
    span: Span,
}

impl Token {
    /// Create a new token out of its string value and its location.
    pub fn new<T: Into<String>>(value: T, span: Span) -> Token {
        Token {
            value: value.into(),
            span: span,
        }
    }

    /// Attempt to parse this token into another type. Like a normal `str`,
    /// `parse()` can parse any type that implements the `FromStr` trait.
    ///
    /// # Examples
    /// ```
    /// # use lishp::lexer::{Token, Span};
    /// let tok = Token::new("1.23", Span::new(0, 3));
    /// let got = tok.parse::<f64>();
    /// assert_eq!(got, Ok(1.23));
    /// ```
    pub fn parse<F>(&self) -> Result<F, F::Err>
        where F: FromStr
    {
        self.value.parse()
    }
}


/// The struct in charge of tokenizing source code.
#[derive(Debug)]
pub struct Lexer {
    source: String,
    position: usize,
    patterns: Vec<Regex>,
}

impl Lexer {
    /// Create a new lexer.
    pub fn new<T: Into<String>>(src: T) -> Lexer {
        Lexer {
            source: src.into(),
            position: 0,
            patterns: make_patterns(),
        }
    }

    /// Get the next token in the stream.
    pub fn next_token(&mut self) -> Result<Option<Token>, InvalidTokenError> {
        if self.position >= self.source.len() {
            return Ok(None);
        }

        for pattern in &self.patterns {
            if let Some((start, end)) = pattern.find(&self.source[self.position..]) {
                // Turn start/end from relative to absolute (true) indices
                let (start, end) = (start + self.position, end + self.position);
                let tok = Token::new(&self.source[start..end], Span::new(start, end));

                if cfg!(test) {
                    // Add a little tracer to the lexer to see what it's matching
                    // TODO: Remove this
                    println!("{} ({}, {}) => {:?}", self.position, start, end, tok);
                }

                self.position = end;
                return Ok(Some(tok));
            }
        }

        Err(InvalidTokenError { pos: self.position })
    }
}

/// Compile all the valid token patterns ahead of time.
fn make_patterns() -> Vec<Regex> {
    let mut patterns = vec![];
    patterns.push(Regex::new(r"^\d+(\.\d+)?").unwrap());  // floats
    patterns.push(Regex::new(r"^-?\d+").unwrap());  // integers
    patterns.push(Regex::new(r"^\(").unwrap());
    patterns.push(Regex::new(r"^\)").unwrap());
    patterns.push(Regex::new(r"^[-_a-zA-Z+=*^&$!@/?|][-_a-zA-Z0-9+=*^&$!@/?|]*").unwrap());  // All valid identifiers
    patterns.push(Regex::new(r#"^"([^\\"]|\\.)*""#).unwrap()); // Double quote strings
    patterns.push(Regex::new(r"(?m)^;.*$").unwrap());  // comments
    patterns.push(Regex::new(r"^\s+").unwrap());
    patterns
}


#[cfg(test)]
mod tests {
    use super::*;

    /// A simple helper macro that generates a test which will read in a bunch
    /// of (src, should_be) pairs and assert that the value the lexer gives is
    /// what it should be.
    macro_rules! lexer_match {
        ($name:ident, $( $src:expr => $should_be:expr ),* ) => {
            #[test]
            fn $name() {
                let inputs = vec![$( ($src , $should_be) ),*];

                for (src, should_be) in inputs {
                    let mut lexer = Lexer::new(src);
                    let got = lexer.next_token();
                    assert_eq!(got, Ok(Some(should_be)));
                }
            }
        };
    }

    macro_rules! token_stream {
        ($name:ident, $( $src:expr => [ $( $should_be: expr ),* ] ),* ) => {
            #[test]
            fn $name() {
                let inputs = vec![$( ($src ,
                    vec![ $( $should_be ),*] )
                    ),*
                ];

                for (src, should_be) in inputs {
                    let mut lexer = Lexer::new(src);
                    let mut got = vec![];

                    for _ in 0..should_be.len() {
                        if let Ok(thing) = lexer.next_token() {
                            got.push(thing);
                        }
                    }

                    for (got, should_be) in got.into_iter().zip(should_be) {
                        assert_eq!(got, Some(should_be));
                    }
                }
            }
        };
    }

    macro_rules! tok {
        ($t:expr) => {
            Token::new($t, Span::new(0, $t.len()))
        };
        ($t:expr, $len:expr) => {
            Token::new($t, Span::new($len, $len + $t.len()))
        };
    }

    lexer_match!(match_numbers,
        "1" => tok!("1"),
        "1.0" => tok!("1.0"));

    lexer_match!(match_whitespace,
        " "    => tok!(" "),
        "   "  => tok!("   "),
        "\n"   => tok!("\n"),
        "\n\t" => tok!("\n\t"),
        "\t"   => tok!("\t"));

    lexer_match!(match_identifiers,
        "foo" => tok!("foo"),
        "FOO" => tok!("FOO"),
        "l33t" => tok!("l33t"),
        "f_" => tok!("f_"),
        "_f" => tok!("_f"),
        "_" => tok!("_"),
        "+" => tok!("+"),
        "-" => tok!("-"),
        "/" => tok!("/"),
        "=" => tok!("="),
        "?" => tok!("?"),
        "!" => tok!("!"),
        "$" => tok!("$"),
        "@" => tok!("@"),
        "*" => tok!("*"),
        "&" => tok!("&"),
        "|" => tok!("|"),
        "$ARGV$" => tok!("$ARGV$")
    );

    #[test]
    fn empty_source() {
        let src = "";
        let mut lexer = Lexer::new(src);
        let got = lexer.next_token();
        assert_eq!(got, Ok(None));
    }

    token_stream!(multi_token_streams,
        "()" => [tok!("("), tok!(")", 1)],

        "(1 2)" => [tok!("("),
                    tok!("1", 1),
                    tok!(" ", 2),
                    tok!("2", 3),
                    tok!(")", 4)],

        "(+ a -42)" => [tok!("("),
                        tok!("+", 1),
                        tok!(" ", 2),
                        tok!("a", 3),
                        tok!(" ", 4),
                        tok!("-42", 5),
                        tok!(")", 8)]
    );

    token_stream!(comments,
        "; aasd" => [tok!("; aasd")],

        // check comments work when there's other stuff on the line too
        "() ; foo" => [tok!("("),
                       tok!(")", 1),
                       tok!(" ", 2),
                       tok!("; foo", 3)],

        // make sure comments only go to the end of the line
        ";comment\n(stuff )" => [tok!(";comment"),
                                 tok!("\n", 8),
                                 tok!("(", 9),
                                 tok!("stuff", 10),
                                 tok!(" ", 15),
                                 tok!(")", 16)]
    );
}
