use std::collections::HashMap;

use crate::language_type::Type;

#[derive(Clone)]
pub struct Enviroment {
    variables: HashMap<String, Type>, 
}

impl Enviroment {
    pub fn new() -> Self{
       Enviroment {
           variables: HashMap::new(),
       } 
    }
    
    pub fn set(&mut self, name: String, value: Type) {
        self.variables.insert(name, value);      
    }  
    
    pub fn get(&mut self, name: String) -> Type {
        self.variables.get(name.as_str()).unwrap().clone()
    }
}