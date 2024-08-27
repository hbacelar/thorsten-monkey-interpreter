#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Token<'a> {
    pub kind: TokenKind,
    pub val: &'a str,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TokenKind {
    //String
    String,

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
    Ident,
    Int,
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
