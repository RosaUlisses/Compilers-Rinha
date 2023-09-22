use std::arch::x86_64::_xgetbv;
use std::collections::HashMap;
use std::sync::Mutex;
use crate::function::Function;
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
    
    pub fn erase(&mut self, name: String) {
        self.variables.remove(name.as_str());
    }
}