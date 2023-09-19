use std::sync::Mutex;
use crate::function::Function;
pub enum Type<'a> {
    Int(i32),
    Str(String),
    Bool(bool),
    Tuple((Box<Type<'a>>, Box<Type<'a>>)),
    Function(Function<'a>),
}

impl<'a> Clone for Type<'a> {
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

impl<'a> Type<'a> {
    pub fn to_string(&self) -> String {
        match self {
            Type::Int(value) => value.to_string(),
            Type::Str(value) => value.clone(),
            Type::Bool(value) => value.to_string(),
            Type::Tuple(value) => format!("({},{})", value.0.to_string(), value.1.to_string()),
            Type::Function(value) => String::from("<#closure>"),
        }
    }
    
    pub fn is_truthy(&self) -> bool {
        match self {
            Type::Bool(value) => value.clone(),
            _ => false
        }
    }
    
    pub fn to_function(&self) -> Option<&'a mut Function> {
        match self {
            Type::Function(value) => Some(*value),
            _ => None
        }
    }

    pub fn to_tuple(&self) -> Option<(Type, Type)> {
        match self {
            Type::Tuple(value) => Some((value.0.as_ref().clone(), value.1.as_ref().clone())),
            _ => None
        }
    }

    pub fn add(left_value: &mut Type, right_value: &mut Type) -> Option<Type<'a>> {
        match (left_value, right_value) {
            (Type::Str(left), Type::Str(right)) => Some(Type::Str(left.clone() + right.as_str())),
            (Type::Int(left), Type::Str(right)) => Some(Type::Str(left.to_string() + right.as_str())),
            (Type::Str(left), Type::Int(right)) => Some(Type::Str(left.clone() + right.to_string().as_str())),
            (Type::Int(left), Type::Int(right)) => Some(Type::Int(left.clone() + right.clone())),
            _ => None
        }
    }

    pub fn sub(left_value: &mut Type, right_value: &mut Type) -> Option<Type<'a>> {
        match (left_value, right_value) {
            (Type::Int(left), Type::Int(right)) => Some(Type::Int(left.clone() + right.clone())),
            _ => None
        }
    }

    pub fn mul(left_value: &mut Type, right_value: &mut Type) -> Option<Type<'a>> {
        match (left_value, right_value) {
            (Type::Int(left), Type::Int(right)) => Some(Type::Int(left.clone() * right.clone())),
            _ => None
        }
    }

    pub fn div(left_value: &mut Type, right_value: &mut Type) -> Option<Type<'a>> {
        match (left_value, right_value) {
            (Type::Int(left), Type::Int(right)) => Some(Type::Int(left.clone() / right.clone())),
            _ => None
        }
    }

    pub fn remainder(left_value: &mut Type, right_value: &mut Type) -> Option<Type<'a>> {
        match (left_value, right_value) {
            (Type::Int(left), Type::Int(right)) => Some(Type::Int(left.clone() % right.clone())),
            _ => None
        }
    }

    pub fn equal(left_value: &mut Type, right_value: &mut Type) -> bool {
        match (left_value, right_value) {
            (Type::Int(left), Type::Int(right)) => left.clone() == right.clone(),
            (Type::Str(left), Type::Str(right)) => left.clone() == right.clone(),
            (Type::Bool(left), Type::Bool(right)) => left.clone() == right.clone(),
            _ => false
        }
    }

    pub fn not_equal(left_value: &mut Type, right_value: &mut Type) -> bool {
        !Type::equal(left_value, right_value)
    }

    pub fn less_than(left_value: &mut Type, right_value: &mut Type) -> bool {
        match (left_value, right_value) {
            (Type::Int(left), Type::Int(right)) => left.clone() < right.clone(),
            _ => false
        }
    }

    pub fn greater_than(left_value: &mut Type, right_value: &mut Type) -> bool {
        match (left_value, right_value) {
            (Type::Int(left), Type::Int(right)) => left.clone() > right.clone(),
            _ => false
        }
    }

    pub fn less_than_equal(left_value: &mut Type, right_value: &mut Type) -> bool {
        Type::less_than(left_value, right_value) || Type::equal(left_value, right_value)
    }

    pub fn greater_than_equal(left_value: &mut Type, right_value: &mut Type) -> bool {
        Type::greater_than(left_value, right_value) || Type::equal(left_value, right_value)
    }

    pub fn and(left_value: &mut Type, right_value: &mut Type) -> bool {
        left_value.is_truthy() && right_value.is_truthy()
    }

    pub fn or(left_value: &mut Type, right_value: &mut Type) -> bool {
        left_value.is_truthy() || right_value.is_truthy()
    }
}