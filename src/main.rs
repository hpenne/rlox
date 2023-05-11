mod error_reporter;
mod evaluate_expr;
mod expr;
mod parser;
mod scanner;
mod token;
mod token_type;

use crate::error_reporter::ErrorReporter;
use crate::evaluate_expr::EvaluateExpr;
use crate::parser::Parser;
use crate::scanner::TokenScanner;
use crate::token::Token;
use std::cell::RefCell;
use std::rc::Rc;
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
        let error = Rc::new(RefCell::new(ErrorReporter::default()));
        run(&line.as_ref().unwrap(), error);
        println!("{}", line.unwrap());
    }
}

fn run_file(file: &str) {
    println!("File: {}", file);
    match fs::read_to_string(file) {
        Ok(source) => {
            let error = Rc::new(RefCell::new(ErrorReporter::default()));
            run(&source, error.clone());
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

fn run(source: &str, error: Rc<RefCell<ErrorReporter>>) {
    let mut parser = Parser::new(source.chars().tokens(error.clone()), error);
    if let Some(expr) = parser.parse() {
        let result = expr.evaluate().unwrap();
        println!("{} = {}", expr, result);
    }
}

mod test {
    use crate::error_reporter::ErrorReporter;
    use crate::evaluate_expr::EvaluateExpr;
    use crate::expr::LiteralValue;
    use crate::parser::Parser;
    use crate::scanner::TokenScanner;
    use std::cell::RefCell;
    use std::rc::Rc;

    fn evaluate_string(text: &str) -> LiteralValue {
        let error_reporter = Rc::new(RefCell::new(ErrorReporter::default()));
        let mut parser = Parser::new(text.chars().tokens(error_reporter.clone()), error_reporter);
        parser.parse().unwrap().evaluate().unwrap()
    }

    #[test]
    fn evaluate_float_addition() {
        assert_eq!(LiteralValue::Number(3.0), evaluate_string("1+2"));
    }

    #[test]
    fn evaluate_float_expr() {
        assert_eq!(LiteralValue::Number(17.0), evaluate_string("3*(6+4)/2+2"));
    }
}
