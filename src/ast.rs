use std::fmt::Display;

use anyhow::bail;

use crate::token::{Token, TokenKind};

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
    Block(BlockStatement),
}

#[derive(Debug, PartialEq, Eq)]
pub enum Expression {
    Callable(CallableExpression),
    IntegerLiteral(IntegerLiteral),
    BooleanLiteral(BooleanLiteral),
    Prefix(PrefixExpression),
    Infix(InfixExpression),
    If(IfExpression),
    Call(CallExpression),
}

#[derive(Debug, PartialEq, Eq)]
pub enum CallableExpression {
    Identifier(Identifier),
    FunctionLiteral(FunctionLiteral),
}

#[derive(Debug, PartialEq, Eq)]
pub struct CallExpression {
    pub func: CallableExpression,
    pub arguments: Vec<Expression>,
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
pub struct BlockStatement {
    pub statements: Vec<Statement>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Identifier {
    pub value: String,
}

#[derive(Debug, PartialEq, Eq)]
pub struct IntegerLiteral {
    pub value: i64,
}

#[derive(Debug, PartialEq, Eq)]
pub struct BooleanLiteral {
    pub value: bool,
}

#[derive(Debug, PartialEq, Eq)]
pub struct FunctionLiteral {
    pub body: BlockStatement,
    pub parameters: Vec<Identifier>,
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
pub struct IfExpression {
    pub condition: Box<Expression>,
    pub consequence: BlockStatement,
    pub alternative: Option<BlockStatement>,
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
    Gt,
    Lparen,
}

impl Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operator::Minus => write!(f, "-"),
            Operator::Plus => write!(f, "+"),
            Operator::Bang => write!(f, "!"),
            Operator::Asterisk => write!(f, "*"),
            Operator::Slash => write!(f, "/"),
            Operator::Eq => write!(f, "="),
            Operator::NotEq => write!(f, "!="),
            Operator::Lt => write!(f, "<"),
            Operator::Gt => write!(f, ">"),
            Operator::Lparen => write!(f, "("),
        }
    }
}

impl TryFrom<&Token<'_>> for Operator {
    type Error = anyhow::Error;

    fn try_from(value: &Token) -> Result<Self, Self::Error> {
        match value.kind {
            TokenKind::Plus => Ok(Operator::Plus),
            TokenKind::Minus => Ok(Operator::Minus),
            TokenKind::Bang => Ok(Operator::Bang),
            TokenKind::Asterisk => Ok(Operator::Asterisk),
            TokenKind::Slash => Ok(Operator::Slash),
            TokenKind::Lt => Ok(Operator::Lt),
            TokenKind::Gt => Ok(Operator::Gt),
            TokenKind::Eq => Ok(Operator::Eq),
            TokenKind::NotEq => Ok(Operator::NotEq),
            TokenKind::Lparen => Ok(Operator::Lparen),
            _ => bail!("Token cannot be converted into operator"),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Node {
    Program(Program),
    Statement(Statement),
    Expression(Expression),
}
