use anyhow::bail;

use crate::token::Token;

#[derive(Debug, PartialEq, Eq)]
pub struct Program {
    pub statments: Vec<Statement>,
    pub errors: Vec<String>,
}

impl Default for Program {
    fn default() -> Self {
        Self::new()
    }
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
    BooleanLiteral(BooleanLiteral),
    PrefixExpression(PrefixExpression),
    InfixExpression(InfixExpression),
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
pub struct BooleanLiteral {
    pub value: bool,
}

#[derive(Debug, PartialEq, Eq)]
pub struct PrefixExpression {
    // TODO use arenas, and vec based index on nodes
    pub right: Box<Expression>,
    // Only allow prefix operators bang, minus
    pub operator: Operator,
}

#[derive(Debug, PartialEq, Eq)]
pub struct InfixExpression {
    // TODO use arenas, and vec based index on nodes
    pub left: Box<Expression>,
    pub right: Box<Expression>,
    pub operator: Operator,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Operator {
    Minus,
    Plus,
    Bang,
    Asterisk,
    Slash,
    Eq,
    NotEq,
    Lt,
    Gt
}

impl TryFrom<&Token> for Operator {
    type Error = anyhow::Error;

    fn try_from(value: &Token) -> Result<Self, Self::Error> {
        match value {
            Token::Plus => Ok( Operator::Plus),
            Token::Minus => Ok(Operator::Minus),
            Token::Bang => Ok(Operator::Bang),
            Token::Asterisk => Ok(Operator::Asterisk),
            Token::Slash => Ok(Operator::Slash),
            Token::Lt => Ok(Operator::Lt),
            Token::Gt => Ok(Operator::Gt),
            Token::Eq => Ok(Operator::Eq),
            Token::NotEq => Ok(Operator::NotEq),
            _ => bail!("Token cannot be converted into operator")
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Node {
    Program(Program),
    Statement(Statement),
    Expression(Expression),
}
