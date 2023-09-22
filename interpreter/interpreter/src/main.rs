use std::fs;
use std::path::Path;
use crate::interpreter::Interpreter;
use crate::language_type::Type::Int;

mod parser;
mod expression;
mod interpreter;
mod enviroment;
mod language_type;
mod function;

fn main() {
    let path = Path::new("./asts/fib.json");
    let json_string = fs::read_to_string(path).expect("The file must exists.");
    let ast_json = json::parse(json_string.as_str()).unwrap();
    let parsed = parser::parse_file(&ast_json).as_ref().to_owned();
    let mut interpreter = Interpreter::new(); 
    interpreter.interpret(parsed);
    
}
