use std::sync::Mutex;
use crate::enviroment::Enviroment;
use crate::expression::Expression;
use crate::language_type::Type;

#[derive(Clone)]
pub struct Function {
   pub parameters: Vec<String>, 
   pub body: Expression,
}

impl Function {
   pub fn new(parameters: Vec<String>, body: Expression) -> Self {
       Function {
           parameters,
           body,  
       }
   } 
}