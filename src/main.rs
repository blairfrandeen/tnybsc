use std::path::PathBuf;

use clap::Parser;
pub mod lexer;
pub mod parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long)]
    source: Option<PathBuf>,

    #[clap(short, long)]
    code: Option<String>,
}

fn main() {
    let args = Args::parse();
    if let Some(code) = args.code {
        let tok = lexer::lex_source(&code);
        dbg!(&tok);
        let mut prgm = parser::Program::new();
        prgm.build(tok.unwrap());
        dbg!(prgm);
    } else if let Some(source_path) = args.source {
        match std::fs::read_to_string(source_path) {
            Ok(file) => {
                let tok = lexer::lex_source(&file);
                dbg!(&tok);
                let mut prgm = parser::Program::new();
                match prgm.build(tok.unwrap()) {
                    Ok(()) => {
                        dbg!(prgm);
                    }
                    Err(err) => panic!("{:?}", err),
                }
            }
            Err(err) => panic!("could not open file: {:?}", err),
        }
    }
}
