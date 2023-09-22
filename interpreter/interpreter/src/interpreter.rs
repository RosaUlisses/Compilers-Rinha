use std::sync::Mutex;
use crate::expression::{Expression, LiteralValue, BinaryOperator};
use crate::enviroment::{Enviroment};
use crate::function;
use crate::language_type::Type;
use crate::function::{Function};
use crate::language_type::Type::Str;

pub struct Interpreter {
    enviroment: Enviroment,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter { enviroment: Enviroment::new() }
    }

    pub fn interpret(&mut self, expression: Expression) {
        self.eval(expression); 
    }

    pub fn eval(&mut self, expression: Expression) -> Type {
        match expression {
            Expression::File {expr} => self.eval_file(expr), 
            Expression::If {condition, then_branch, else_branch} => self.eval_if(condition, then_branch, else_branch),
            Expression::Lambda {parameters, body} => self.eval_function_declaration(parameters, body),
            Expression::Call {callee, arguments} => self.eval_call(callee, arguments),
            Expression::VarDeclaration {name, value, next} => self.eval_var_declaration(name, value, next),
            Expression::Binary {left, operator, right} => self.eval_binary(left, operator, right),
            Expression::Tuple {first, second} => self.eval_tuple(first, second),
            Expression::VarExpression {name} => self.eval_var(name),
            Expression::Literal {value} => self.eval_literal(value),
            Expression::Print {value} => self.eval_print(value),
            Expression::First {tuple} => self.eval_first(tuple),
            Expression::Second {tuple} => self.eval_second(tuple),
        }
    }

    fn eval_file(&mut self, expression: Box<Expression>) -> Type {
        self.eval(*expression) 
    }

    fn eval_if(&mut self, condition: Box<Expression>, then_branch: Box<Expression>, else_branch: Box<Expression>) -> Type {
        let condition: bool = self.eval(*condition).is_truthy(); 
        
        if condition {
            self.eval(*then_branch)
        } else {  
           self.eval(*else_branch) 
        } 
    }

    fn eval_function_declaration<'b>(&mut self, parameters: Vec<String>, body: Box<Expression>) -> Type {
        Type::Function(Function::new(parameters, body.as_ref().clone()))
    }

    fn eval_call(&mut self, callee: Box<Expression>, arguments: Vec<Box<Expression>>) -> Type {
        let outer_scope_enviroment: Enviroment = self.enviroment.clone();
        let function: Function = self.eval(*callee).to_function().unwrap();

        let mut evaluated_arguments: Vec<Type> = vec![];
        for argument in arguments {
            evaluated_arguments.push(self.eval(*argument));    
        }

        for (parameter_name, parameter_value) in function.parameters.iter().zip(evaluated_arguments.iter_mut()) {
            self.enviroment.set(parameter_name.clone(), parameter_value.clone())   
        }
        let value_to_return = self.eval(function.body);
        self.enviroment = outer_scope_enviroment;
        
        value_to_return
    }

    fn eval_var_declaration(&mut self, name: String, value: Box<Expression>, next: Box<Expression>) -> Type {
        let name: String = name.clone(); 
        let value: Type = self.eval(*value);
        self.enviroment.set(name, value.clone());
        
        self.eval(*next)
    }
    
    fn eval_binary(&mut self, left: Box<Expression>, operator: BinaryOperator, right: Box<Expression>) -> Type {
        let mut left: Type = self.eval(*left);
        let mut right: Type = self.eval(*right);
        
        let result = match operator { 
            BinaryOperator::Addition => Type::add(&mut left, &mut right).unwrap(),
            BinaryOperator::Subtraction => Type::sub(&mut left, &mut right).unwrap(),
            BinaryOperator::Multiplication => Type::mul(&mut left, &mut right).unwrap(),
            BinaryOperator::Division => Type::div(&mut left, &mut right).unwrap(),
            BinaryOperator::Remainder => Type::remainder(&mut left, &mut right).unwrap(),
            BinaryOperator::Equal =>  Type::Bool(Type::equal(&mut left, &mut right)),
            BinaryOperator::NotEqual => Type::Bool(Type::not_equal(&mut left, &mut right)),
            BinaryOperator::GreaterThan => Type::Bool(Type::greater_than(&mut left, &mut right)),
            BinaryOperator::LessThan => Type::Bool(Type::less_than(&mut left, &mut right)),
            BinaryOperator::GreaterThanEqual => Type::Bool(Type::greater_than_equal(&mut left, &mut right)),
            BinaryOperator::LessThanEqual => Type::Bool(Type::less_than_equal(&mut left, &mut right)),
            BinaryOperator::And => Type::Bool(Type::and(&mut left, &mut right)),
            BinaryOperator::Or => Type::Bool(Type::or(&mut left, &mut right)),
        };
        
        result.clone()
    }
    
    
    
    fn eval_tuple(&mut self, first: Box<Expression>, second: Box<Expression>) -> Type {
        let first: Type =  self.eval(*first);
        let second: Type = self.eval(*second);
        
        Type::Tuple((Box::new(first.clone()), Box::new(second.clone())))
    }
    
    fn eval_var(&mut self, name: String) -> Type {
        self.enviroment.get(name).clone()
    }
    
    fn eval_literal(&mut self, literal_value: LiteralValue) -> Type {
       match literal_value {
           LiteralValue::Str(value) => Type::Str(value.clone()),
           LiteralValue::Int(value) => Type::Int(value.clone()),
           LiteralValue::Bool(value) => Type::Bool(value.clone()),
       }
    }
    
    fn eval_print(&mut self, value: Box<Expression>) -> Type {
        let string: String = self.eval(*value).to_string();
        println!("{}", string);
        
        Type::Str(string)
    }
    
    fn eval_first(&mut self, tuple: Box<Expression>) -> Type {
        self.eval(*tuple).to_tuple().unwrap().0
    }

    fn eval_second(&mut self, tuple: Box<Expression>) -> Type {
        self.eval(*tuple).to_tuple().unwrap().1
    }
}