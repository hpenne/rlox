extern crate core;

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
    println!("Usage: rlox <script>");
}

fn run_prompt(input: impl BufRead, mut output: impl Write) {
    let mut environment = Rc::new(RefCell::new(Environment::default()));
    for line in input.lines() {
        let error = Rc::new(RefCell::new(ErrorReporter::default()));
        run(
            line.as_ref().unwrap(),
            &mut environment,
            &error,
            &mut output,
        );
    }
}

fn run_file(file: &str) {
    let mut environment = Rc::new(RefCell::new(Environment::default()));
    println!("File: {file}");
    match fs::read_to_string(file) {
        Ok(source) => {
            let error = Rc::new(RefCell::new(ErrorReporter::default()));
            run(&source, &mut environment, &error, &mut io::stdout());
            if error.borrow().has_error() {
                std::process::exit(65);
            }
        }
        Err(e) => {
            println!("Failed to read from file: {e}");
            std::process::exit(1);
        }
    }
}

fn run(
    source: &str,
    environment: &mut Rc<RefCell<Environment>>,
    error: &Rc<RefCell<ErrorReporter>>,
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
    use crate::environment::Environment;
    use crate::error_reporter::ErrorReporter;
    use std::cell::RefCell;
    use std::rc::Rc;

    fn run(input: &str) -> String {
        let mut environment = Rc::new(RefCell::new(Environment::default()));
        let mut output = Vec::new();
        let error = Rc::new(RefCell::new(ErrorReporter::default()));
        crate::run(&input, &mut environment, &error, &mut output);
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

    #[test]
    fn assignment() {
        assert_eq!(
            run("
                var a = 1;
                a = 2;
                a = 3;
                print a;
                "),
            "3\n"
        );
    }

    #[test]
    fn block() {
        assert_eq!(
            run("
                var b = 1;
                print b;
                {
                    var b = 2;
                    print b;
                }
                "),
            "1\n2\n"
        );
    }
}
