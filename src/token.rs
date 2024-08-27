#[derive(Debug, PartialEq, Eq)]
pub enum Token<'a> {
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
    Ident(&'a str),
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
