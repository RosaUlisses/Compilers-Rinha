use crate::enviroment::Enviroment;
use crate::expression::Expression;
use crate::language_type::Type;
struct Function {
   parameters: Vec<String>, 
   body: Type,
   closure: Box<Enviroment> 
}

impl Function {
   pub fn new(parameters: Vec<String>, body: Type, closure: Box<Enviroment>) -> Self {
       Function {
           parameters,
           body,  
           closure
       }
   } 
}