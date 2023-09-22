#[derive(Clone)]
pub enum BinaryOperator {
    Addition,
    Subtraction,
    Multiplication,
    Division,
    Remainder,
    Equal,
    NotEqual,
    GreaterThan,
    LessThan,
    GreaterThanEqual,
    LessThanEqual,
    And,
    Or,
}

impl BinaryOperator {
   pub fn from_string(operator: &str) -> Option<BinaryOperator> {
        match operator { 
            "Add" => Some(BinaryOperator::Addition),
            "Sub" => Some(BinaryOperator::Subtraction),
            "Mul" => Some(BinaryOperator::Multiplication),
            "Div" => Some(BinaryOperator::Division),
            "Rem" => Some(BinaryOperator::Remainder),
            "Eq" => Some(BinaryOperator::Equal),
            "Neq" => Some(BinaryOperator::NotEqual),
            "Gt" => Some(BinaryOperator::GreaterThan),
            "Lt" => Some(BinaryOperator::LessThan),
            "Gte" => Some(BinaryOperator::GreaterThanEqual),
            "Lte" => Some(BinaryOperator::LessThanEqual),
            "And" => Some(BinaryOperator::And),
            "Or" => Some(BinaryOperator::Or),
            _ => None
        }  
   } 
}

#[derive(Clone)]
pub enum LiteralValue {
   Int(i32), 
   Str(String),
   Bool(bool), 
}

#[derive(Clone)]
pub enum Expression {
    File {expr: Box<Expression>},
    If {condition: Box<Expression>, then_branch: Box<Expression>, else_branch: Box<Expression>},
    Lambda {parameters: Vec<String>, body: Box<Expression>},
    Call {callee: Box<Expression>, arguments: Vec<Box<Expression>>},
    VarDeclaration {name: String, value: Box<Expression>, next: Box<Expression>},
    Binary {left: Box<Expression>, operator: BinaryOperator, right: Box<Expression>},
    Tuple {first: Box<Expression>, second: Box<Expression>},
    VarExpression {name: String},
    Literal {value: LiteralValue},
    Print {value: Box<Expression>},
    First {tuple: Box<Expression>},
    Second {tuple: Box<Expression>}
}