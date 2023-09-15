use std::arch::x86_64::__cpuid;
use std::os::windows::prelude::BorrowedSocket;
use crate::expression::Expression;
use json::JsonValue;
use json::object;
use crate::expression::LiteralValue::Str;

const KIND: &str = "kind";

enum ExpressionKind {
    Int,
    Str,
    Let,
    Var,
    Function,
    Tuple,
    Call,
    Parameter,
    File,
    If,
    Bool,
    First,
    Second,
    Print,
}

impl ExpressionKind {
    
   pub fn from_str(kind: &str) -> Option<ExpressionKind> {
      match (kind)  { 
          "Int" => Some(ExpressionKind::Int),
          "Str" => Some(ExpressionKind::Str),
          "Var" => Some(ExpressionKind::Var),
          "Function" => Some(ExpressionKind::Function),
          "Tuple" => Some(ExpressionKind::Tuple),
          "Call" => Some(ExpressionKind::Call),
          "Parameter" => Some(ExpressionKind::Parameter),
          "File" => Some(ExpressionKind::File),
          "If" => Some(ExpressionKind::If),
          "Bool" => Some(ExpressionKind::Bool),
          "First" => Some(ExpressionKind::First),
          "Second" => Some(ExpressionKind::Second),
            _ =>  None  
      } 
   } 
}

fn parse_int(expression_ast: &JsonValue) -> Box<Expression> {
    Box::default()    
}

fn parse_str(expression_ast: &JsonValue) -> Box<Expression>{
    Box::default()
}

fn parse_var(expression_ast: &JsonValue) -> Box<Expression>{
    Box::default()
}

fn parse_function_declaration(expression_ast: &JsonValue) -> Box<Expression>{
    Box::default()
}

fn parse_tuple(expression_ast: &JsonValue) -> Box<Expression>{
    Box::default()
}

fn parse_call(expression_ast: &JsonValue) -> Box<Expression>{
    Box::default()
}

fn parse_parameter(expression_ast: &JsonValue) -> Box<Expression>{
    Box::default()
}

fn parse_file(expression_ast: &JsonValue) -> Box<Expression>{
    Box::default()
}

fn parse_if(expression_ast: &JsonValue) -> Box<Expression>{
    Box::default()
}

fn parse_bool(expression_ast: &JsonValue) -> Box<Expression>{
    Box::default()
}

fn parse_first(expression_ast: &JsonValue) -> Box<Expression>{
    Box::default()
}

fn parse_second(expression_ast: &JsonValue) -> Box<Expression>{
    Box::default()
}

pub fn parse(expression_ast: &JsonValue) -> Box<Expression> {
    let expression_kind = ExpressionKind::from_str(expression_ast[KIND].as_str().unwrap());     
    
    match expression_kind {
        Some(ExpressionKind::Int) => parse_int(expression_ast),
        Some(ExpressionKind::Str) => parse_str(expression_ast),
        Some(ExpressionKind::Str) => parse_str(expression_ast),
    } 
    
}