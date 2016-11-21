extern crate lishp;

use std::env::args;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::process::exit;

const USAGE: &'static str = "interpreter <file>";

fn main() {
    let filename = args().nth(1).unwrap_or_else(|| {
        println!("USAGE: {}", USAGE);
        exit(1)
    });

    let path = PathBuf::from(filename);

    let mut src = String::new();
    let mut f = File::open(path).unwrap();
    f.read_to_string(&mut src).unwrap();

    let tokens = match lishp::tokenize(src) {
        Ok(tokens) => tokens,
        Err(e) => {
            println!("Syntax Error: {:?}", e);
            exit(1);
        }
    };

    let ast = match lishp::parse(tokens) {
        Ok(ast) => ast,
        Err(e) => {
            println!("Parsing Error: {:?}", e);
            exit(1);
        }
    };

    // TODO: Run `eval` on the AST to start the actual interpreting.
}
