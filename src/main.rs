mod scanner;
mod token;
mod tokentype;
mod object;
mod ast;
mod error;
// mod parser;
mod interpreter;
mod parser;


use std::env::args;
use std::fs;
use colored::Colorize;
use crate::error::ScrapError;
use crate::error::ScrapError::RuntimeError;
use crate::interpreter::Interpreter;
use crate::parser::Parser;
// use crate::parser::Parser;
use crate::scanner::Scanner;
use crate::token::Token;
use std::time::Instant;


fn main(){

    std::env::set_var("RUST_BACKTRACE", "5");

    let input: Vec<String> = args().collect();
    if input.len() < 3 {
        error::ScrapError::error(ScrapError::RuntimeError, "too few arguments", 1, file!())
    }
    if input[1] == "scrap" {
        run_file(input[2].clone());
    }
struct Scrap {}


fn run_file(source: String) {

    // println!("{:?}", source);
    let input = fs::read_to_string(&source);
    if input.is_err() {
        ScrapError::error(
            RuntimeError,
            format!("unable to read file {}", source).as_str(),
            line!() as usize,
            file!()
        )
    } else {
        run(input.unwrap())
    }

}
fn run(input: String) {
    let now = Instant::now();
    let mut scanner = Scanner::new(input);
    scanner.scan_tokens();

    let mut parser = Parser::new(scanner.tokens);
    parser.parse();
    /*for stmt in &parser.statements {
        let f = format!("\n{:#?}\n", stmt).yellow();
        println!("{}", f);
    };*/

    let mut interpreter = Interpreter::new(parser.expressions, parser.statements);
    interpreter.start();
    let formatted_time = format!("{}", now.elapsed().as_secs_f64()).bright_magenta();
    println!("{formatted_time}");
}}