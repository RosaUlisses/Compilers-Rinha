#[derive(Copy, Clone)]
pub enum Type {
    Int(i32),
    Str(String),
    Bool(bool),
    Tuple((Type, Type))
}

impl Type {
    pub fn to_string(&self) -> String {
        match self {
            Type::Int(value) => value.to_string(),
            Type::Str(value) => value,
            Type::Bool(value) => value.to_string(),
            Type::Tuple(value) => format!("({},{})", value.0.to_string(), value.1.to_string())
        }
    }
    
    pub fn is_truthy(&self) -> bool {
        match self {
            Type::Bool(value) => value.clone(),
            _ => false
        }
    }

    pub fn to_tuple(&self) -> Option<&(Type, Type)> {
        match self {
            Type::Tuple(value) => Some(value),
            _ => None
        }
    }

    pub fn add(left_value: Type, right_value: Type) -> Option<Type> {
        match (left_value, right_value) {
            (Type::Str(left), Type::Str(right)) => Some(Type::Str(concat!(left.as_str(), right.as_str()).to_string())),
            (Type::Int(left), Type::Str(right)) => Some(Type::Str(concat!(left.as_str(), right.as_str()).to_string())),
            (Type::Str(left), Type::Int(right)) => Some(Type::Str(concat!(left.as_str(), right.as_str()).to_string())),
            (Type::Int(left), Type::Int(right)) => Some(Type::Int(left + right)),
            _ => None
        }
    }

    pub fn sub(left_value: Type, right_value: Type) -> Option<Type> {
        match (left_value, right_value) {
            (Type::Int(left), Type::Int(right)) => Some(Type::Int(left + right)),
            _ => None
        }
    }

    pub fn mul(left_value: Type, right_value: Type) -> Option<Type> {
        match (left_value, right_value) {
            (Type::Int(left), Type::Int(right)) => Some(Type::Int(left * right)),
            _ => None
        }
    }

    pub fn div(left_value: Type, right_value: Type) -> Option<Type> {
        match (left_value, right_value) {
            (Type::Int(left), Type::Int(right)) => Some(Type::Int(left / right)),
            _ => None
        }
    }

    pub fn remainder(left_value: Type, right_value: Type) -> Option<Type> {
        match (left_value, right_value) {
            (Type::Int(left), Type::Int(right)) => Some(Type::Int(left % right)),
            _ => None
        }
    }

    pub fn equal(left_value: Type, right_value: Type) -> bool {
        match (left_value, right_value) {
            (Type::Int(left), Type::Int(right)) => left == right,
            (Type::Str(left), Type::Str(right)) => left == right,
            (Type::Bool(left), Type::Bool(right)) => left == right,
            _ => false
        }
    }

    pub fn not_equal(left_value: Type, right_value: Type) -> bool {
        !Type::equal(left_value, right_value)
    }

    pub fn less_than(left_value: Type, right_value: Type) -> bool {
        match (left_value, right_value) {
            (Type::Int(left), Type::Int(right)) => left < right,
            _ => false
        }
    }

    pub fn greater_than(left_value: Type, right_value: Type) -> bool {
        match (left_value, right_value) {
            (Type::Int(left), Type::Int(right)) => left > right,
            _ => false
        }
    }

    pub fn less_than_equal(left_value: Type, right_value: Type) -> bool {
        Type::less_than(left_value, right_value) || Type::equal(left_value, right_value)
    }

    pub fn greater_than_equal(left_value: Type, right_value: Type) -> bool {
        Type::greater_than(left_value, right_value) || Type::equal(left_value, right_value)
    }

    pub fn and(left_value: Type, right_value: Type) -> bool {
        left_value.is_truthy() && right_value.is_truthy()
    }

    pub fn or(left_value: Type, right_value: Type) -> bool {
        left_value.is_truthy() || right_value.is_truthy()
    }
}