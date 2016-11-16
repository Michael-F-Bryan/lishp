extern crate lishp;


fn main() {
    let src = "(1 2 )";
    let got = lishp::parse(src);
    println!("{}", got.unwrap());
}
