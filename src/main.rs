use std::env;
use std::fs;
use std::process;

mod regex_parser;
mod nfa;
mod dfa;
mod lexer_generator;
mod spec_parser;

use spec_parser::parse_spec;
use lexer_generator::generate_lexer;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <spec_file>", args[0]);
        process::exit(1);
    }

    let spec_file = &args[1];

    // Reads spec file
    let spec_content = match fs::read_to_string(spec_file) {
        Ok(content) => content,
        Err(err) => {
            eprintln!("Error reading spec file '{}': {}", spec_file, err);
            process::exit(1);
        }
    };

    // Parse the spec
    let spec = match parse_spec(&spec_content) {
        Ok(spec) => spec,
        Err(err) => {
            eprintln!("Error parsing spec: {}", err);
            process::exit(1);
        }
    };

    // Generate the lexer
    match generate_lexer(&spec) {
        Ok(_) => {
            println!("Lexer generated successfully");
        }
        Err(err) => {
            eprintln!("Error generating lexer: {}", err);
            process::exit(1);
        }
    }
}
