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
    LetStatement(LetStatement),
    ReturnStatement(ReturnStatement),
}

#[derive(Debug, PartialEq, Eq)]
pub enum Expression {
    Identifier(Identifier),
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
pub struct Identifier {
    pub value: String,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Node {
    Program(Program),
    Statement(Statement),
    Expression(Expression),
}
