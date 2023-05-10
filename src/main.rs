mod error_reporter;
mod expr;
mod parser;
mod scanner;
mod token;
mod token_type;

use crate::error_reporter::ErrorReporter;
use crate::scanner::TokenScanner;
use crate::token::Token;
use std::{env, fs, io};

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        1 => run_prompt(),
        2 => run_file(&args[1]),
        _ => {
            print_help();
        }
    };
}

fn print_help() {
    println!("Usage: rlox <script>")
}

fn run_prompt() {
    for line in io::stdin().lines() {
        let mut error = ErrorReporter::default();
        run(&line.as_ref().unwrap(), &mut error);
        println!("{}", line.unwrap());
    }
}

fn run_file(file: &str) {
    println!("File: {}", file);
    match fs::read_to_string(file) {
        Ok(source) => {
            let mut error = ErrorReporter::default();
            run(&source, &mut error);
            if error.has_error() {
                std::process::exit(65);
            }
        }
        Err(e) => {
            println!("Failed to read from file: {}", e);
            std::process::exit(1);
        }
    }
}

fn run(source: &str, error: &mut ErrorReporter) {
    for token in source.chars().tokens(error) {
        println!("{}", token);
    }
}
