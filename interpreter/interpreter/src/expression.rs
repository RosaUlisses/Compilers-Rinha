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
   Bool(bool), 
}

pub enum Expression {
    File {expression: Box<Expression>},
    If {condition: Box<Expression>, then_branch: Box<Expression>, else_branch: Box<Expression>},
    FunctionDeclaration {parameters: Vec<String>, body: Box<Expression>},
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