use std::{env, io};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let args : Vec<String> = env::args().collect();
    let _result = match args.len() {
        1 =>run_prompt(),
        2 => run_file(&args[1]),
        _ => {
            print_help();
            Ok(())
        },
    };
}

fn print_help(){
    println!("Usage: rlox <script>")
}

fn run_prompt()  -> Result<(), Box<dyn std::error::Error>>{
    for line in io::stdin().lines() {
        println!("{}", line.unwrap());
    }
    Ok(())
}

fn run_file(file: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("File: {}", file);
    let file = File::open(file)?;
    let reader = BufReader::new(file);
    for line in reader.lines() {
        run(&line?)?;
    }
    Ok(())
}

fn run(line: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", line);
    Ok(())
}