use crate::expression::{Expression, LiteralValue};
use json::{JsonValue, value};

const KIND: &str = "kind";
const VALUE: &str = "value";
const CONDITION: &str = "condition";
const THEN_BRANCH: &str = "then";
const ELSE_BRANCH: &str = "otherwise";
const VARIABLE_NAME: &str = "text";
const PARAMETERS: &str = "parameters";
const FUNCTION_BODY: &str = "value";
const PARAMETER_NAME: &str = "text";
const CALLEE: &str = "callee";
const ARGUMENTS: &str = "arguments";
const TEXT: &str = "text";
const NEXT: &str = "next";
const TUPLE_FIRST_ELEMENT: &str = "first";
const TUPLE_SECOND_ELEMENT: &str = "second";
const TUPLE_VALUE: &str = "value";
const EXPRESSION: &str = "expression";

enum ExpressionKind {
    Int,
    Str,
    Let,
    Var,
    Function,
    Tuple,
    Call,
    File,
    If,
    Bool,
    First,
    Second,
    Print,
}

impl ExpressionKind {
   pub fn from_str(kind: &str) -> Option<ExpressionKind> {
      match kind { 
          "Int" => Some(ExpressionKind::Int),
          "Str" => Some(ExpressionKind::Str),
          "Var" => Some(ExpressionKind::Var),
          "Let" => Some(ExpressionKind::Let),
          "Function" => Some(ExpressionKind::Function),
          "Tuple" => Some(ExpressionKind::Tuple),
          "Call" => Some(ExpressionKind::Call),
          "File" => Some(ExpressionKind::File),
          "If" => Some(ExpressionKind::If),
          "Bool" => Some(ExpressionKind::Bool),
          "First" => Some(ExpressionKind::First),
          "Second" => Some(ExpressionKind::Second),
          "Print" => Some(ExpressionKind::Print),
            None =>  None  
      } 
   } 
}

pub fn parse(expression_ast: &JsonValue) -> Box<Expression> {
    let expression_kind = ExpressionKind::from_str(expression_ast[KIND].as_str().unwrap()).unwrap();     
    
    match expression_kind {
        ExpressionKind::Int => parse_int_literal(expression_ast),
        ExpressionKind::Str => parse_string_literal(expression_ast),
        ExpressionKind::Var => parse_var(expression_ast),
        ExpressionKind::Let => parse_var_declaration(expression_ast),
        ExpressionKind::Function => parse_function_declaration(expression_ast),
        ExpressionKind::Tuple => parse_tuple(expression_ast),
        ExpressionKind::Tuple => parse_tuple(expression_ast),
        ExpressionKind::call => parse_call(expression_ast),
        ExpressionKind::File => parse_file(expression_ast),
        ExpressionKind::If => parse_if(expression_ast),
        ExpressionKind::Bool => parse_bool_literal(expression_ast),
        ExpressionKind::First => parse_first(expression_ast),
        ExpressionKind::Second => parse_second(expression_ast),
        ExpressionKind::Print => parse_second(expression_ast),
    } 
    
}

fn parse_file(expression_ast: &JsonValue) -> Box<Expression> {
    let expression: Box<Expression> = parse(&expression_ast[EXPRESSION]);
    
    Box::new(Expression::File {expression})
}

fn  parse_print(expression_ast: &JsonValue) -> Box<Expression> {
    let value: Box<Expression> = parse(&expression_ast[VALUE]);
    
    Box::new(Expression::Print {value})
}

fn parse_first(expression_ast: &JsonValue) -> Box<Expression> {
    let tuple: Box<Expression> = parse(&expression_ast[TUPLE_VALUE]);  
    
    Box::new(Expression::First {tuple})
}

fn parse_second(expression_ast: &JsonValue) -> Box<Expression> {
    let tuple: Box<Expression> = parse(&expression_ast[TUPLE_VALUE]);

    Box::new(Expression::Second {tuple})
}

fn parse_tuple(expression_ast: &JsonValue) -> Box<Expression> {
    let first: Box<Expression> = parse(&expression_ast[TUPLE_FIRST_ELEMENT]);
    let second: Box<Expression> = parse(&expression_ast[TUPLE_SECOND_ELEMENT]);

    Box::new(Expression::Tuple {first, second})
}

fn parse_call(expression_ast: &JsonValue) -> Box<Expression> {
    let callee: Box<Expression> = parse(&expression_ast[CALLEE]);

    let arguments: Vec<Box<Expression>> = expression_ast[ARGUMENTS].members()
        .map(|argument| parse(argument))
        .collect();

    Box::new(Expression::Call {callee, arguments})
}

fn parse_function_declaration(expression_ast: &JsonValue) -> Box<Expression> {
    let parameters: Vec<String> = expression_ast[PARAMETERS].members()
        .map(|parameter| parameter[PARAMETER_NAME].take_string().unwrap())
        .collect();

    let body: Box<Expression> = parse(&expression_ast[FUNCTION_BODY]);

    Box::new(Expression::FunctionDeclaration {parameters, body})
}

fn parse_var(expression_ast: &JsonValue) -> Box<Expression> {
    let name = expression_ast[VARIABLE_NAME].take_string().unwrap();

    Box::new(Expression::VarExpression {name})
}

fn parse_var_declaration(expression_ast: &JsonValue) -> Box<Expression> {
    let name: String = expression_ast[VARIABLE_NAME][TEXT].take_string().unwrap();
    let value: Box<Expression> = parse(&expression_ast[VALUE]);
    let next: Box<Expression> = parse(&expression_ast[NEXT]);

    Box::new(Expression::VarDeclaration {name, value, next})
}

fn parse_if(expression_ast: &JsonValue) -> Box<Expression> {
    let condition: Box<Expression> = parse(&expression_ast[CONDITION]);
    let then_branch: Box<Expression> = parse(&expression_ast[THEN_BRANCH]);
    let else_branch: Box<Expression> = parse(&expression_ast[ELSE_BRANCH]);

    Box::new(Expression::If {condition, then_branch, else_branch})
}

fn parse_string_literal(expression_ast: &JsonValue) -> Box<Expression> {
    let value: String = expression_ast[VALUE].take_string().unwrap();

    Box::new(Expression::Literal {value: LiteralValue::Str(value)})
}

fn parse_int_literal(expression_ast: &JsonValue) -> Box<Expression> {
    let value: i32 = expression_ast[VALUE].as_i32().unwrap();

    Box::new(Expression::Literal {value: LiteralValue::Int(value)})
}

fn parse_bool_literal(expression_ast: &JsonValue) -> Box<Expression> {
    let value: bool = expression_ast[VALUE].as_bool().unwrap();

    Box::new(Expression::Literal {value: LiteralValue::Bool(value)})
}