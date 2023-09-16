use std::collections::HashMap;
use std::sync::Mutex;

pub enum RinhaType {
    Int(i32),
    Str(String)
}

pub struct Enviroment {
    variables: HashMap<String, RinhaType>, 
    enclosing: Option<Box<Enviroment>> 
}

impl Enviroment {
    pub fn new() -> Self{
       Enviroment {
           variables: HashMap::new(),
           enclosing: None 
       } 
    }
    
    pub fn set_enclosing(&mut self, enclosing: Box<Enviroment>) {
        self.enclosing = Some(enclosing); 
    }
    
    pub fn set(&mut self, name: String, value: RinhaType) {
        self.variables.insert(name, value);      
    }  
    
    pub fn get(&mut self, name: String) -> RinhaType{
        self.variables.get(name).unwrap()
    }
    
}