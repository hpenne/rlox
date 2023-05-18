mod environment;
mod error_reporter;
mod evaluate_expr;
mod exec_stmt;
mod expr;
mod parser;
mod scanner;
mod statement;
mod token;
mod token_type;

use crate::environment::Environment;
use crate::error_reporter::ErrorReporter;
use crate::exec_stmt::ExecuteStatement;
use crate::parser::Parser;
use crate::scanner::TokenScanner;
use crate::token::Token;
use std::cell::RefCell;
use std::io::{BufRead, BufReader, Write};
use std::rc::Rc;
use std::{env, fs, io};

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        1 => run_prompt(BufReader::new(io::stdin()), io::stdout()),
        2 => run_file(&args[1]),
        _ => {
            print_help();
        }
    };
}

fn print_help() {
    println!("Usage: rlox <script>")
}

fn run_prompt(input: impl BufRead, mut output: impl Write) {
    let mut environment = Environment::default();
    for line in input.lines() {
        let error = Rc::new(RefCell::new(ErrorReporter::default()));
        run(
            &line.as_ref().unwrap(),
            &mut environment,
            error,
            &mut output,
        );
    }
}

fn run_file(file: &str) {
    let mut environment = Environment::default();
    println!("File: {}", file);
    match fs::read_to_string(file) {
        Ok(source) => {
            let error = Rc::new(RefCell::new(ErrorReporter::default()));
            run(&source, &mut environment, error.clone(), &mut io::stdout());
            if error.borrow().has_error() {
                std::process::exit(65);
            }
        }
        Err(e) => {
            println!("Failed to read from file: {}", e);
            std::process::exit(1);
        }
    }
}

fn run(
    source: &str,
    environment: &mut Environment,
    error: Rc<RefCell<ErrorReporter>>,
    output: &mut impl Write,
) {
    let mut parser = Parser::new(source.chars().tokens(error.clone()), error.clone());
    let statements = parser.parse();
    if !error.borrow().has_error() {
        for statement in statements {
            if let Err(error) = statement.execute(environment, output) {
                write!(output, "Runtime error: {}", error.message).unwrap();
                break;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::run_prompt;

    fn run(input: &str) -> String {
        let mut output = Vec::new();
        run_prompt(input.as_bytes(), &mut output);
        let s = std::str::from_utf8(output.as_ref()).unwrap();
        s.to_string()
    }

    #[test]
    fn print_hello_world() {
        assert_eq!(run("print \"Hello World!\";"), "Hello World!\n");
    }

    #[test]
    fn print_expression() {
        assert_eq!(run("print 1+2*3-(2+4)/3;"), "5\n");
    }

    #[test]
    fn print_string_expression() {
        assert_eq!(run("print \"Hello \"+\"World!\";"), "Hello World!\n");
    }

    #[test]
    fn variable() {
        assert_eq!(run("var a = 3; print a;"), "3\n");
    }
}
