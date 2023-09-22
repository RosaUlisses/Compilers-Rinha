use crate::function::Function;
pub enum Type {
    Int(i32),
    Str(String),
    Bool(bool),
    Tuple((Box<Type>, Box<Type>)),
    Function(Function),
}

impl Clone for Type {
    fn clone(&self) -> Self {
        match self  { 
           Type::Int(value) => Type::Int(value.clone()),
           Type::Str(value) => Type::Str(value.clone()),
           Type::Bool(value) => Type::Bool(value.clone()),
           Type::Tuple(value) => Type::Tuple((value.0.clone(), value.1.clone())),
           Type::Function(value) => Type::Function(value.clone()), 
        }
    } 
}

impl Type {
    pub fn to_string(&self) -> String {
        match self {
            Type::Int(value) => value.to_string(),
            Type::Str(value) => value.clone(),
            Type::Bool(value) => value.to_string(),
            Type::Tuple(value) => format!("({},{})", value.0.to_string(), value.1.to_string()),
            Type::Function(_) => String::from("<#closure>"),
        }
    }
    
    pub fn is_truthy(&self) -> bool {
        match self {
            Type::Bool(value) => value.clone(),
            _ => false
        }
    }
    
    pub fn to_function(&self) -> Option<Function> {
        match self {
            Type::Function(value) => Some(value.clone()),
            _ => None
        }
    }

    pub fn to_tuple(&self) -> Option<(Type, Type)> {
        match self {
            Type::Tuple(value) => Some((value.0.as_ref().clone(), value.1.as_ref().clone())),
            _ => None
        }
    }
    
    pub fn add(left_value: Type, right_value: Type) -> Option<Type> {
        match (left_value, right_value) {
            (Type::Str(left), Type::Str(right)) => Some(Type::Str(left.clone() + right.as_str())),
            (Type::Int(left), Type::Str(right)) => Some(Type::Str(left.to_string() + right.as_str())),
            (Type::Str(left), Type::Int(right)) => Some(Type::Str(left.clone() + right.to_string().as_str())),
            (Type::Int(left), Type::Int(right)) => Some(Type::Int(left.clone() + right.clone())),
            _ => None
        }
    }

    pub fn sub(left_value: Type, right_value: Type) -> Option<Type> {
        match (left_value, right_value) {
            (Type::Int(left), Type::Int(right)) => Some(Type::Int(left.clone() - right.clone())),
            _ => None
        }
    }

    pub fn mul(left_value: Type, right_value: Type) -> Option<Type> {
        match (left_value, right_value) {
            (Type::Int(left), Type::Int(right)) => Some(Type::Int(left.clone() * right.clone())),
            _ => None
        }
    }

    pub fn div(left_value: Type, right_value: Type) -> Option<Type> {
        match (left_value, right_value) {
            (Type::Int(left), Type::Int(right)) => Some(Type::Int(left.clone() / right.clone())),
            _ => None
        }
    }

    pub fn remainder(left_value: Type, right_value: Type) -> Option<Type> {
        match (left_value, right_value) {
            (Type::Int(left), Type::Int(right)) => Some(Type::Int(left.clone() % right.clone())),
            _ => None
        }
    }

    pub fn equal(left_value: Type, right_value: Type) -> bool {
        match (left_value, right_value) {
            (Type::Int(left), Type::Int(right)) => left.clone() == right.clone(),
            (Type::Str(left), Type::Str(right)) => left.clone() == right.clone(),
            (Type::Bool(left), Type::Bool(right)) => left.clone() == right.clone(),
            _ => false
        }
    }

    pub fn not_equal(left_value: Type, right_value: Type) -> bool {
        !Type::equal(left_value, right_value)
    }

    pub fn less_than(left_value: Type, right_value: Type) -> bool {
        match (left_value, right_value) {
            (Type::Int(left), Type::Int(right)) => left.clone() < right.clone(),
            _ => false
        }
    }

    pub fn greater_than(left_value: Type, right_value: Type) -> bool {
        match (left_value, right_value) {
            (Type::Int(left), Type::Int(right)) => left.clone() > right.clone(),
            _ => false
        }
    }

    pub fn less_than_equal(left_value: Type, right_value: Type) -> bool {
        match (left_value, right_value) {
            (Type::Int(left), Type::Int(right)) => left.clone() <= right.clone(),
            _ => false
        }
    }

    pub fn greater_than_equal(left_value: Type, right_value: Type) -> bool {
        match (left_value, right_value) {
            (Type::Int(left), Type::Int(right)) => left.clone() >= right.clone(),
            _ => false
        }
    }

    pub fn and(left_value: Type, right_value: Type) -> bool {
        left_value.is_truthy() && right_value.is_truthy()
    }

    pub fn or(left_value: Type, right_value: Type) -> bool {
        left_value.is_truthy() || right_value.is_truthy()
    }
}