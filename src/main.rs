use std::path::PathBuf;

use clap::Parser;
pub mod lexer;

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
        dbg!(tok);
    } else if let Some(source_path) = args.source {
        match std::fs::read_to_string(source_path) {
            Ok(file) => {
                let tok = lexer::lex_source(&file);
                dbg!(tok);
            }
            Err(err) => panic!("could not open file: {:?}", err),
        }
    }
}
