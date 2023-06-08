extern crate core;

use std::cell::RefCell;
use std::io::{BufRead, BufReader, Write};
use std::rc::Rc;
use std::{env, fs, io};

use crate::builtins::add_builtin_functions;
use crate::environment::Environment;
use crate::error_reporter::ErrorReporter;
use crate::exec_stmt::{ErrorOrReturn, ExecuteStatement};
use crate::interpreter::Interpreter;
use crate::parser::Parser;
use crate::scanner::TokenScanner;
use crate::token::Token;

mod builtins;
mod environment;
mod error_reporter;
mod evaluate_expr;
mod exec_stmt;
mod expr;
mod interpreter;
mod literal_value;
mod lox_callable;
mod parser;
mod resolver;
mod scanner;
mod statement;
mod token;
mod token_type;

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

fn create_env() -> Rc<RefCell<Environment>> {
    let environment = Rc::new(RefCell::new(Environment::default()));
    add_builtin_functions(&mut (*environment).borrow_mut());
    environment
}

fn run_prompt(input: impl BufRead, mut output: impl Write) {
    let mut environment = create_env();
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
    let mut environment = create_env();
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
    let mut interpreter = Interpreter {
        globals: environment.clone(),
        resolver: resolver::resolve(&statements, error),
        output,
    };
    if !error.borrow().has_error() {
        for statement in statements {
            if let Err(ErrorOrReturn::Error(error)) =
                statement.execute(environment, &mut interpreter)
            {
                write!(output, "Runtime error: {}", error.message).unwrap();
                break;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use std::cell::RefCell;
    use std::rc::Rc;

    use crate::create_env;
    use crate::error_reporter::ErrorReporter;

    fn run(input: &str) -> String {
        let mut environment = create_env();
        let mut output = Vec::new();
        let error = Rc::new(RefCell::new(ErrorReporter::default()));
        crate::run(input, &mut environment, &error, &mut output);
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
    fn and_operator() {
        assert_eq!(
            run("
                print false and false;
                print false and true;
                print true and false;
                print true and true;
                "),
            "false\nfalse\nfalse\ntrue\n"
        );
    }

    #[test]
    fn or_operator() {
        assert_eq!(
            run("
                print false or false;
                print false or true;
                print true or false;
                print true or true;
                "),
            "false\ntrue\ntrue\ntrue\n"
        );
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

    #[test]
    fn if_block() {
        assert_eq!(
            run("
                var b = 1;
                if (b == 1)
                    print \"Yes\";
                if (b < 1) {
                    print \"No\";
                } else {
                    print \"Yes\";
                }
                "),
            "Yes\nYes\n"
        );
    }

    #[test]
    fn while_loop() {
        assert_eq!(
            run("
                var b = 3;
                while (b > 0) {
                    print b;
                    b = b - 1;
                }
                "),
            "3\n2\n1\n"
        );
    }

    #[test]
    fn for_loop() {
        assert_eq!(
            run("
                var a = 0;
                var temp;
                for (var b = 1; a < 100; b = temp + b) {
                    print a;
                    temp = a;
                    a = b;
                }
                "),
            "0\n1\n1\n2\n3\n5\n8\n13\n21\n34\n55\n89\n"
        );
    }

    #[test]
    fn while_as_for_loop() {
        assert_eq!(
            run("
                var a = 0;
                var temp;
                var b = 1;
                while (a < 100) {
                    print a;
                    temp = a;
                    a = b;
                    b = temp + b;
                }
                "),
            "0\n1\n1\n2\n3\n5\n8\n13\n21\n34\n55\n89\n"
        );
    }

    #[test]
    fn clock() {
        run("print clock();");
    }

    #[test]
    fn func() {
        assert_eq!(
            run("
                fun foo() { 
                    print \"foo\"; 
                    return 5;
                }
                print foo();
                "),
            "foo\n5\n"
        );
    }

    #[test]
    fn fib() {
        assert_eq!(
            run("
                fun fib(n) {
                    if (n <= 1) return n;
                    return fib(n-2) + fib(n-1);
                }
                for (var i = 0; i < 20; i = i + 1) {
                    print fib(i);
                }
            "),
            "0\n1\n1\n2\n3\n5\n8\n13\n21\n34\n55\n89\n144\n233\n377\n610\n987\n1597\n2584\n4181\n"
        );
    }

    #[test]
    fn simple_loop() {
        assert_eq!(
            run("
                for (var i = 0; i < 10; i = i + 1) {
                    print i;
                }
            "),
            "0\n1\n2\n3\n4\n5\n6\n7\n8\n9\n"
        );
    }
}
