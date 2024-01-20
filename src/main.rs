pub mod lexer;

fn main() {
    println!("Hello, world!");
    let tok = lexer::parse_tokens("-+?");
    dbg!(tok);
}
