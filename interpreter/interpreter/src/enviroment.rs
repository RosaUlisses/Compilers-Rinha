use std::collections::HashMap;
use crate::language_type::Type

pub struct Enviroment {
    variables: HashMap<String, Type>, 
    enclosing: Option<Box<&Enviroment>> 
}

impl Enviroment {
    pub fn new() -> Self{
       Enviroment {
           variables: HashMap::new(),
           enclosing: None 
       } 
    }
    
    pub fn set_enclosing(&mut self, enclosing: Box<&Enviroment>) -> Enviroment {
        self.enclosing = Some(enclosing); 
        Self
    }
    
    pub fn set(&mut self, name: String, value: Type) {
        self.variables.insert(name, value);      
    }  
    
    pub fn get(&mut self, name: String) -> Type {
        if self.variables.contains_key(name.as_str()) {
            self.variables.get(name.as_str()).unwrap().to_owned()
        }
        
        else { 
            self.enclosing.unwrap().get(name)
        } 
    }
}