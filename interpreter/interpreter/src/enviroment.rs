use std::arch::x86_64::_xgetbv;
use std::collections::HashMap;
use std::sync::Mutex;
use crate::function::Function;
use crate::language_type::Type;

pub struct Enviroment<'a> {
    variables: HashMap<String, &'a mut Type<'a>>, 
}

impl<'a> Clone for Enviroment<'a> {
    fn clone(&self) -> Self {
        let mut variables: HashMap<String, &'a mut Type<'a>> = HashMap::new();
        let mut  
        for (key, value) in self.variables.into_iter() {
           variables.insert(key, value); 
        }
        
        Enviroment {
            variables
        }
    } 
}


impl<'a> Enviroment<'a> {
    pub fn new() -> Self{
       Enviroment {
           variables: HashMap::new(),
       } 
    }
    
    pub fn set(&mut self, name: String, value: &'a mut Type<'a>) {
        self.variables.insert(name, value);      
    }  
    
    pub fn get(&mut self, name: String) -> &'a mut Type {
        *self.variables.get(name.as_str()).unwrap()
    }
    
    pub fn get_variable_names(&mut self) -> Vec<String> {
        self.variables.keys().map(|key| key.clone()).collect() 
    }
}