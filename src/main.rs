#![feature(iter_advance_by)]
use std::env;
pub mod lexer;

fn main() {
    let mut args = env::args();
    args.next();
    if let Some(source) = args.next() {
        let tok = lexer::parse_tokens(&source);
        dbg!(tok);
    }
}
