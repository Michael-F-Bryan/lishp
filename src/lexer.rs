//! This is the module containing the lexer.

use regex::Regex;
use std::str::FromStr;


/// Turn some source code into a list of Tokens.
pub fn tokenize<T: Into<String>>(src: T) -> Result<Vec<Token>, InvalidTokenError> {
    let mut lexer = Lexer::new(src);
    let mut tokens = vec![];

    loop {
        match lexer.next_token()? {
            None => break,
            Some(t) => {
                if !t.is_whitespace() {
                    tokens.push(t);
                }
            }
        }
    }

    Ok(tokens)
}

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

    /// Check whether the token consists entirely of whitespace.
    pub fn is_whitespace(&self) -> bool {
        self.value.as_str().trim().len() == 0
    }

    /// Get the length of the token string.
    pub fn len(&self) -> usize {
        self.value.len()
    }

    /// Check if the token starts with a particular string.
    pub fn starts_with(&self, pat: &str) -> bool {
        self.value.starts_with(pat)
    }

    /// Check if the token's first character is a number.
    pub fn starts_with_number(&self) -> bool {
        if let Some(digit) = self.value.chars().next() {
            digit.is_digit(10)
        } else {
            false
        }
    }

    /// Get a reference to the Token as a string.
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl PartialEq<str> for Token {
    fn eq(&self, other: &str) -> bool {
        self.value == other.as_ref()
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

    #[test]
    fn tokenizer() {
        let src = "(+ foo bar (9))";
        let should_be = vec![tok!("(", 0),
                             tok!("+", 1),
                             tok!("foo", 3),
                             tok!("bar", 7),
                             tok!("(", 11),
                             tok!("9", 12),
                             tok!(")", 13),
                             tok!(")", 14)];

        let got = tokenize(src);
        assert_eq!(got, Ok(should_be));
    }
}
