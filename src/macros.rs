/// A short-hand way of creating a new token.
macro_rules! tok {
        ($t:expr) => {
            {
                use lexer::{Token, Span};
                Token::new($t, Span::new(0, $t.len()))
            }
        };
        ($t:expr, $len:expr) => {
            {
                use lexer::{Token, Span};
                Token::new($t, Span::new($len, $len + $t.len()))
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
                    let mut lexer = Lexer::new(src);
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
