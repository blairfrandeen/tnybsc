use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;

use clap::{CommandFactory, Parser};

use crate::compiler::{emitter, lexer, parser};

pub mod compiler;

/// A tiny basic compiler
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Execute code from the specified path
    #[clap(value_parser)]
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

    let program_name: String;
    let source_code = if let Some(code) = args.code {
        // add newline automatically
        let mut source = code;
        source.push('\n');
        program_name = "terminal_commands".to_string();
        source
    } else if let Some(source_path) = args.source_path {
        program_name = source_path
            .file_stem()
            .expect("file name was passed!")
            .to_os_string()
            .into_string()
            .unwrap();
        match std::fs::read_to_string(source_path) {
            Ok(file) => file,
            Err(err) => panic!("could not open file: {:?}", err),
        }
    } else {
        Args::command().print_help().unwrap();
        std::process::exit(1);
    };

    let tokens = lexer::lex_source(&source_code);
    if let Some(lex_opt) = args.lex {
        if let Some(_lex_path) = lex_opt {
            todo!("output of scanned code to file not implemented")
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
        if let Some(_parse_path) = parse_opt {
            todo!("output of parsed code to file not implemented")
        } else {
            dbg!(&prgm);
        }
    }
    let mut emitter = emitter::Emitter::new();
    emitter.build(prgm);

    let mut compile_path = PathBuf::from("./artifacts");
    compile_path.push(&program_name);

    if let Some(compile_opt) = args.compile {
        if let Some(compile_opt_arg) = compile_opt {
            compile_path = PathBuf::from(compile_opt_arg);
        } else {
            print!("{}", emitter);
        }
    }
    let mut build_path = compile_path.clone();
    compile_path.set_extension("c");
    build_path.set_extension("out");
    let mut compiled_file = match fs::File::create(&compile_path) {
        Ok(path) => path,
        Err(err) => panic!("unable to create {:?}: {err:?}", &compile_path),
    };
    match compiled_file.write_all(format!("{emitter}").as_bytes()) {
        Ok(_) => {}
        Err(err) => panic!("unable to write to {:?}: {err:?}", &compile_path),
    };

    let compile_command = Command::new("gcc")
        .arg(compile_path)
        .arg("-o")
        .arg(&build_path)
        .output();
    match compile_command {
        Ok(out) => {
            for line in String::from_utf8(out.stdout).unwrap().lines() {
                println!("{}", line);
            }
            for line in String::from_utf8(out.stderr).unwrap().lines() {
                println!("{}", line);
            }
        }
        Err(err) => panic!("Error during compilation: {}", err),
    };

    let run_command = Command::new(build_path).output();
    match run_command {
        Ok(out) => {
            for line in String::from_utf8(out.stdout).unwrap().lines() {
                println!("{}", line);
            }
        }
        Err(err) => panic!("Error during code execution: {}", err),
    };
}
