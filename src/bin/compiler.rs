//! This isn't really a compiler, instead what it'll do is read in a source
//! file, embed that into a generated Rust program, then use `rustc` to compile
//! that program into a binary.

extern crate lishp;
extern crate tempfile;

use std::env::args;
use std::fs::File;
use std::io::Result as IoResult;
use std::path::PathBuf;
use std::process::{exit, Output, Command};

use tempfile::{NamedTempFile, NamedTempFileOptions};

const USAGE: &'static str = "compiler <file>";


fn main() {
    let filename = args().nth(1).unwrap_or_else(|| {
        println!("USAGE: {}", USAGE);
        exit(1);
    });

    let path = PathBuf::from(filename);
    let src = render(path);
    println!("{}", src);

    let outfile = write_to_file(src).expect("Failed to create a temp file");
    println!("{:?}", outfile);

    compile(outfile.path().to_str().unwrap(),
            "/home/michael/Documents/lishp/target/debug",
            "/home/michael/Documents/lishp/target/debug/deps",
            "/tmp/foo.exe");
}

fn render(filename: PathBuf) -> String {
    use std::fmt::Write;

    let mut buf = String::new();
    // add the extern crate line and ignore all lints
    writeln!(buf, "#![allow(unused_variables)]");
    writeln!(buf, "extern crate lishp;");

    // then include the target's contents and embed it in the binary
    writeln!(buf,
             "const BINARY: &'static str = include_str!(\"{}\");",
             filename.display());

    // Finally, write out the rest of the program
    writeln!(buf,
             "{}",
             r#"
fn main() {
    let tokens = lishp::tokenize(BINARY).expect("Unable to tokenize file");
    let mut parser = lishp::Parser::new(tokens);
    let ast = parser.parse().expect("Failed to parse file");
}
"#);

    buf
}

fn write_to_file(src: String) -> IoResult<NamedTempFile> {
    use std::io::Write;
    let mut f = NamedTempFileOptions::new().prefix("").suffix(".rs").create()?;
    write!(f, "{}", src);
    Ok(f)
}

fn compile<T: Into<String>>(filename: T, deps: T, lishp_library: T, to: T) -> Output {
    Command::new("rustc")
        .args(&["-L", &deps.into()])
        .args(&["-L", &lishp_library.into()])
        .args(&["-o", &to.into()])
        .arg(filename.into())
        .output()
        .expect("Failed to run rustc")
}
