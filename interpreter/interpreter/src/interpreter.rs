use crate::expression::{Expression, LiteralValue};
use crate::enviroment::{Enviroment, RinhaType};

struct Interpreter {
    expression: Expression,
    enviroment: Enviroment,
}

impl Interpreter {
    pub fn new(expression: Expression) -> Self {
        Interpreter { expression: expression, enviroment: Enviroment::new() }
    }

    pub fn interpret() {}

    fn eval(expression: &Expression) {
        match expression {
            Expression::File => {}
            Expression::If { .. } => {}
            Expression::FunctionDeclaration { .. } => {}
            Expression::Call { .. } => {}
            Expression::VarDeclaration { .. } => {}
            Expression::Binary { .. } => {}
            Expression::Tuple { .. } => {}
            Expression::VarExpression { .. } => {}
            Expression::Literal { .. } => {}
            Expression::Print { .. } => {}
            Expression::First { .. } => {}
            Expression::Second { .. } => {}
        }
    }

    fn eval_file(expression: &Expression) {}

    fn eval_if(expression: &Expression) {}

    fn eval_function_declaration(expression: &Expression) {}

    fn eval_call(expression: &Expression) {}

    fn eval_var_declaration(expression: &Expression) {}
    
    fn eval_binary(expression: &Expression) {}
    
    fn eval_tuple(expression: &Expression) {}
    
    fn eval_var(expression: &Expression) {}
    
    fn eval_literal(expression: &Expression) {}
    
    fn eval_print(expression: &Expression) {}
    
    fn eval_first(expression: &Expression) {}
    
    fn eval_second(expression: &Expression) {}
}