use std::ops::DerefMut;
use crate::expression::{Expression, LiteralValue, BinaryOperator};
use crate::enviroment::{Enviroment};
use crate::language_type::Type;

struct Interpreter {
    expression: Expression,
    enviroment: Box<Enviroment>,
}

impl Interpreter {
    pub fn new(expression: Expression) -> Self {
        Interpreter { expression: expression, enviroment: Box::new(Enviroment::new()) }
    }

    pub fn interpret() {}

    pub fn eval(&mut self, expression: &Expression) -> Type {
        // match expression {
        //     Expression::File => {}
        //     Expression::If { .. } => {}
        //     Expression::FunctionDeclaration { .. } => {}
        //     Expression::Call { .. } => {}
        //     Expression::VarDeclaration { .. } => {}
        //     Expression::Binary { .. } => {}
        //     Expression::Tuple { .. } => {}
        //     Expression::VarExpression { .. } => {}
        //     Expression::Literal { .. } => {}
        //     Expression::Print { .. } => {}
        //     Expression::First { .. } => {}
        //     Expression::Second { .. } => {}
        // }
    }

    fn eval_file(expression: &Expression) {}

    fn eval_if(&mut self, expression: &Expression::If) -> Type {
        let condition: bool = self.eval(expression.condition).is_truthy(); 
        
        if condition {
            self.eval(expression.then_branch)
        } else {  
           self.eval(expression.else_branch) 
        } 
    }

    fn eval_function_declaration(&mut self, expression: &Expression::Lambda) {
        let mut function_enviroment: Enviroment = Enviroment::new();
        let  aux = self.enviroment.deref_mut();
        self.enviroment = Box::new(function_enviroment);
        self.enviroment.set_enclosing(Box::new(aux));
    }

    fn eval_call(expression: &Expression) {}

    fn eval_var_declaration(&mut self, expression: &Expression::VarDeclaration) -> Type{
        let name: String = expression.name; 
        let value: Type = self.eval(expression.value);
        self.enviroment.set(name, value);
        
        self.eval(expression.next)
    }
    
    fn eval_binary(&mut self, expression: &Expression::Binary) -> Type {
        let left: Type = self.eval(expression.left);
        let right: Type = self.eval(expression.right);
        
        match expression.operator { 
            BinaryOperator::Addition => Type::add(left, right).unwrap(),
            BinaryOperator::Subtraction => Type::sub(left, right).unwrap(),
            BinaryOperator::Multiplication => Type::mul(left, right).unwrap(),
            BinaryOperator::Division => Type::div(left, right).unwrap(),
            BinaryOperator::Remainder => Type::remainder(left, right).unwrap(),
            BinaryOperator::Equal => Type::Bool(Type::equal(left, right)),
            BinaryOperator::NotEqual => Type::Bool(Type::not_equal(left, right)),
            BinaryOperator::GreaterThan => Type::Bool(Type::greater_than(left, right)),
            BinaryOperator::LessThan => Type::Bool(Type::less_than(left, right)),
            BinaryOperator::GreaterThanEqual => Type::Bool(Type::greater_than_equal(left, right)),
            BinaryOperator::LessThanEqual => Type::Bool(Type::less_than_equal(left, right)),
            BinaryOperator::And => Type::Bool(Type::and(left, right)),
            BinaryOperator::Or => Type::Bool(Type::or(left, right)),
        } 
    }
    
    
    
    fn eval_tuple(&mut self, expression: &Expression::Tuple) -> Type {
        let first: Type =  self.eval(expression.first);
        let second: Type = self.eval(expression.second);
        
        Type::Tuple((first, second))
    }
    
    fn eval_var(&mut self, expression: &Expression::VarExpression) -> Type {
        self.enviroment.get(expression.name)
    }
    
    fn eval_literal(&mut self, expression: &Expression::Literal) -> Type {
       match expression.value {
           LiteralValue::Str(value) => Type::Str(value),
           LiteralValue::Int(value) => Type::Int(value),
           LiteralValue::Bool(value) => Type::Bool(value),
       }
    }
    
    fn eval_print(&mut self, expression: &Expression::Print) -> Type {
        let value: Type = self.eval(expression.value);
        println!("{}", value.to_string());
        
        value
    }
    
    fn eval_first(&mut self, expression: &Expression::First) -> Type {
        let tuple = self.eval(expression.tuple).to_tuple().unwrap();
        
        tuple.0
    }

    fn eval_second(&mut self, expression: &Expression::Second) -> Type {
        let tuple = self.eval(expression.tuple).to_tuple().unwrap();

        tuple.1
    }
}