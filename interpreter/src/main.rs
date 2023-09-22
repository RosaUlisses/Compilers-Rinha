use std::fs;
use std::path::Path;
use json::JsonValue;
use crate::expression::Expression;
use crate::interpreter::Interpreter;

mod parser;
mod expression;
mod interpreter;
mod enviroment;
mod language_type;
mod function;

fn main() {
    let path: &Path = Path::new("./source.rinha.json");
    let json_string: String = fs::read_to_string(path).expect("The file must exists.");
    let ast_json: JsonValue = json::parse(json_string.as_str()).unwrap();
    
    let parsed_ast: Expression = parser::parse_file(&ast_json).as_ref().to_owned();
    let mut interpreter = Interpreter::new(); 
    interpreter.interpret(parsed_ast);
}
