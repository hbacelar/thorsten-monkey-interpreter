#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Token {
    //Keywords
    Let,
    Function,
    True,
    False,
    If,
    Else,
    Return,

    Illegal,
    Eof,
    Ident(String),
    Int(i32),
    Comma,
    Semicolon,
    Lparen,
    Rparen,
    Lbrace,
    Rbrace,

    //Operators
    Assign,
    Plus,
    Minus,
    Bang,
    Asterisk,
    Slash,

    Lt,
    Gt,
    Eq,
    NotEq,
}
