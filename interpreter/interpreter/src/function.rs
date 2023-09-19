use std::sync::Mutex;
use crate::enviroment::Enviroment;
use crate::expression::Expression;
use crate::language_type::Type;

#[derive(Clone)]
pub struct Function<'a> {
   pub parameters: Vec<String>, 
   pub body: &'a Expression,
}

impl<'a> Function<'a> {
   pub fn new(parameters: Vec<String>, body: &'a Expression) -> Self {
       Function {
           parameters,
           body,  
       }
   } 
}