#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    ILLEGAL,
    EOF,
    IDENT(String),
    INT(i32),
    ASSIGN,
    PLUS,
    COMMA,
    SEMICOLON,
    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,
    FUNCTION,
    LET,
}
