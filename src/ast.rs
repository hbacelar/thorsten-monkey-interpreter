#[derive(Debug, PartialEq, Eq)]
pub struct Program {
    pub statments: Vec<Statement>,
    pub errors: Vec<String>,
}

impl Program {
    pub fn new() -> Self {
        Program {
            statments: Vec::new(),
            errors: Vec::new(),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Statement {
    Let(LetStatement),
    Return(ReturnStatement),
    Expression(ExpressionStatement),
}

#[derive(Debug, PartialEq, Eq)]
pub enum Expression {
    Identifier(Identifier),
    IntegerLiteral(IntegerLiteral),
    PrefixExpression(PrefixExpression),
}

#[derive(Debug, PartialEq, Eq)]
pub struct LetStatement {
    pub name: Identifier,
    pub value: Expression,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ReturnStatement {
    pub value: Expression,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ExpressionStatement {
    pub expression: Expression,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Identifier {
    pub value: String,
}

#[derive(Debug, PartialEq, Eq)]
pub struct IntegerLiteral {
    pub value: i32,
}

#[derive(Debug, PartialEq, Eq)]
pub struct PrefixExpression {
    // TODO use arenas, and vec based index on nodes
    pub right: Box<Expression>,
    pub operator: Operator,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Operator {
    Minus,
    Bang,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Node {
    Program(Program),
    Statement(Statement),
    Expression(Expression),
}
