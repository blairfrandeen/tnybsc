use std::path::PathBuf;

use clap::Parser;

use crate::compiler::{emitter, lexer, parser};

pub mod compiler;

/// A tiny basic compiler
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Execute code from the specified path
    #[clap(short, long)]
    source_path: Option<PathBuf>,

    /// Execute code directly from the command line
    #[clap(short, long)]
    code: Option<String>,

    /// Print lexical output to LEX_PATH or STDOUT if path not supplied.
    #[clap(short, long, id = "LEX_PATH")]
    lex: Option<Option<PathBuf>>,

    /// Print parser output to PARSE_PATH or STDOUT if path not supplied.
    #[clap(short, long, id = "PARSE_PATH")]
    parse: Option<Option<PathBuf>>,

    /// Print compiler output to COMPILE_PATH or STDOUT if path not supplied.
    #[clap(long, id = "COMPILE_PATH")]
    compile: Option<Option<PathBuf>>,
}

fn main() {
    let args = Args::parse();
    let source_code = if let Some(code) = args.code {
        // add newline automatically
        let mut source = code;
        source.push('\n');
        source
    } else if let Some(source_path) = args.source_path {
        match std::fs::read_to_string(source_path) {
            Ok(file) => file,
            Err(err) => panic!("could not open file: {:?}", err),
        }
    } else {
        panic!("must supply source code or file!")
    };

    let tokens = lexer::lex_source(&source_code);
    if let Some(lex_opt) = args.lex {
        if let Some(lex_path) = lex_opt {
            todo!()
        } else {
            dbg!(&tokens);
        }
    }
    let mut prgm = parser::Program::new();
    match prgm.build(tokens.unwrap()) {
        Ok(()) => {}
        Err(err) => panic!("{:?}", err),
    }
    if let Some(parse_opt) = args.parse {
        if let Some(parse_path) = parse_opt {
            todo!()
        } else {
            dbg!(&prgm);
        }
    }
    let mut emitter = emitter::Emitter::new();
    emitter.build(prgm);
    if let Some(compile_opt) = args.compile {
        if let Some(compile_path) = compile_opt {
            todo!()
        } else {
            print!("{}", emitter);
        }
    }
}
