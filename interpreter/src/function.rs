use crate::expression::Expression;

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