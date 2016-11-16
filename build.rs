extern crate lalrpop;

fn main() {
    // Tell cargo it only needs to re-generate the parser if it has changed
    println!("cargo:rerun-if-changed=./src/grammar.lalrpop");
    println!("cargo:rerun-if-changed=./src/ast.rs");
    lalrpop::process_root().unwrap();
}
