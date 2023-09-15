enum BinaryOperator {
    Addition,
    Subtraction,
    Multiplication,
    Division,
    Remainder,
    Equal,
    NotEqual,
    LessThan,
    GreaterThanEqual,
    LessThanEqual,
    And,
    Or,
}

pub enum LiteralValue {
   Int(i32), 
   Str(String),
   Boolean(bool), 
}

pub enum Expression {
    File {expression: Box<Expression>},
    If {condition: Box<Expression>, then_branch: Box<Expression>, else_branch: Box<Expression>},
    FunctionDeclaration {parameters: Vec<String>, body: Box<Expression>},
    Call {calee: Box<Expression>, arguments: Vec<Box<Expression>>},
    VarDeclaration {name: String, value: Box<Expression>, next: Box<Expression>},
    Binary {left: Box<Expression>, operator: BinaryOperator, right: Box<Expression>},
    Tuple {first: Box<Expression>, second: Box<Expression>},
    Literal {value: LiteralValue}
}