//! Command-line tool for parsing UP files

use std::env;
use std::fs;
use std::process;

use up::parse;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <file.up>", args[0]);
        process::exit(1);
    }

    let filename = &args[1];
    let content = fs::read_to_string(filename).unwrap_or_else(|err| {
        eprintln!("Error reading file '{}': {}", filename, err);
        process::exit(1);
    });

    match parse(&content) {
        Ok(nodes) => {
            println!("{:#?}", nodes);
        }
        Err(err) => {
            eprintln!("Parse error: {}", err);
            process::exit(1);
        }
    }
}
