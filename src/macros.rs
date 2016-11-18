/// A short-hand way of creating a new token.
///
/// # Examples
///
/// ```
/// # #[macro_use]
/// # extern crate lishp;
/// # use lishp::lexer::{Span, Token};
/// # fn main() {
/// let open_paren = tok!("(");
/// assert_eq!(open_paren, Token::new("(", Span::new(0, 1)));
/// # }
/// ```
#[macro_export]
macro_rules! tok {
        ($t:expr) => {
            {
                $crate::lexer::Token::new($t, $crate::lexer::Span::new(0, $t.len()))
            }
        };
        ($t:expr, $len:expr) => {
            {
                $crate::lexer::Token::new($t, $crate::lexer::Span::new($len, $len + $t.len()))
            }
        };
}


/// A simple helper macro that generates a test which will read in a bunch
/// of (src, should_be) pairs and assert that the value the lexer gives is
/// what it should be.
macro_rules! lexer_match {
        ($name:ident, $( $src:expr => $should_be:expr ),* ) => {
            #[test]
            fn $name() {
                let inputs = vec![$( ($src , $should_be) ),*];

                for (src, should_be) in inputs {
                    let mut lexer = $crate::lexer::Lexer::new(src);
                    let got = lexer.next_token();
                    assert_eq!(got, Ok(Some(should_be)));
                }
            }
        };
    }


/// Pretty much the same as `lexer_match!()`, but this one is designed to
/// validate a list of tokens, not just one.
macro_rules! token_stream {
        ($name:ident, $( $src:expr => [ $( $should_be: expr ),* ] ),* ) => {
            #[test]
            fn $name() {
                let inputs = vec![$( ($src ,
                    vec![ $( $should_be ),*] )
                    ),*
                ];

                for (src, should_be) in inputs {
                    let mut lexer = $crate::lexer::Lexer::new(src);
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


/// Create a Vec of one or more Tokens. The Span is automatically calculated
/// when the token list is generated.
#[macro_export]
macro_rules! toks {
    ( $( $t:expr ),+ ) => {
        {
            let mut tokens = vec![];
            let mut _pos = 0;
            $(
                let next_tok = tok!($t, _pos);
                _pos += next_tok.len();
                tokens.push(next_tok);
            )*

            tokens
        }
    };

    // We need this variant so the `unnecessary mut` lint isn't triggered.
    () => {
        {
            let tokens: Vec<$crate::lexer::Token> = Vec::new();
            tokens
        }
    }
}

/// Short-cut for creating a new Type instance.
#[macro_export]
macro_rules! t {
    (List, [ $( $val:expr ),* ] ) => {
        $crate::types::Type::List(vec![
            $( $val ),*
            ])
    };
    (String, $val:expr) => {
        $crate::types::Type::String($val.to_string())
    };
    (Sym, $val:expr) => {
        $crate::types::Type::Symbol($val.to_string())
    };
    (Int, $val:expr) => {
        $crate::types::Type::Integer($val)
    };
    (Float, $val:expr) => {
        $crate::types::Type::Float($val)
    };
    (Bool, $val:expr) => {
        $crate::types::Type::Boolean($val)
    };
    (Nil) => {
        $crate::types::Type::Nil
    };
}


#[cfg(test)]
mod tests {
    use lexer::{Span, Token};
    use types::Type;

    #[test]
    fn tok_creates_valid_tokens() {
        let inputs = vec![(tok!("("), Token::new("(", Span::new(0, 1))),
                          (tok!("foo"), Token::new("foo", Span::new(0, 3))),
                          (tok!("foo", 5), Token::new("foo", Span::new(5, 8))),
                          (tok!("a", 9), Token::new("a", Span::new(9, 10)))];

        for (got, should_be) in inputs {
            assert_eq!(got, should_be);
        }
    }

    #[test]
    fn create_list_of_tokens_with_toks() {
        let inputs =
            vec![(toks!(), Vec::new()), // empty token list
                 (toks!("1"), vec![tok!("1", 0)]), // single item

                 // Lots of tokens (makes sure span is computed correctly, too)
                 (toks!("(", "+", "1", "1", ")"),
                  vec![tok!("(", 0), tok!("+", 1), tok!("1", 2), tok!("1", 3), tok!(")", 4)])];

        for (got, should_be) in inputs {
            assert_eq!(got, should_be);
        }
    }

    #[test]
    fn create_type_instances() {
        let inputs = vec![(t!(Nil), Type::Nil),
                          (t!(Bool, true), Type::Boolean(true)),
                          (t!(Bool, false), Type::Boolean(false)),
                          (t!(Int, 5), Type::Integer(5)),
                          (t!(Float, 3.14159), Type::Float(3.14159)),
                          (t!(Sym, "foo"), Type::Symbol("foo".to_string())),
                          (t!(String, "foo"), Type::String("foo".to_string())),
                          (t!(List, []), Type::List(vec![])),
                          (t!(List, [t!(Int, 1)]), Type::List(vec![Type::Integer(1)])),
                          (t!(List, [t!(Int, 1), t!(Sym, "bar")]),
                           Type::List(vec![Type::Integer(1), Type::Symbol("bar".to_string())]))];

        for (got, should_be) in inputs {
            assert_eq!(got, should_be);
        }
    }
}
