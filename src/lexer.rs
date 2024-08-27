use crate::token::{Token, TokenKind};

pub struct Lexer<'a> {
    input: &'a str,
    pub position: usize,
    read_position: usize,
    ch: char,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut l = Lexer {
            input,
            position: 0,
            read_position: 0,
            ch: '\0',
        };
        l.read_char();
        l
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = '\0';
        } else {
            self.ch = self
                .input
                .chars()
                .nth(self.read_position)
                .expect("char bound already checked")
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn peak_char(&mut self) -> char {
        if self.read_position >= self.input.len() {
            '\0'
        } else {
            self.input
                .chars()
                .nth(self.read_position)
                .expect("char bound already checked")
        }
    }

    fn read_itentifier(&mut self) -> &'a str {
        let pos = self.position;
        while self.ch.is_alphabetic() {
            self.read_char()
        }
        &self.input[pos..self.position]
    }

    fn read_number(&mut self) -> &'a str {
        let pos = self.position;
        while self.ch.is_numeric() {
            self.read_char()
        }
        &self.input[pos..self.position]
    }

    fn consume_whitespace(&mut self) {
        while self.ch == ' ' || self.ch == '\t' || self.ch == '\n' || self.ch == '\r' {
            self.read_char();
        }
    }

    fn token_from_pos(&self, kind: TokenKind) -> Token<'a> {
        Token {
            kind,
            val: &self.input[self.position..self.read_position],
        }
    }

    pub fn next_token(&mut self) -> Option<Token<'a>> {
        self.consume_whitespace();

        let ch = self.ch;
        let token = match ch {
            '=' => match self.peak_char() {
                '=' => {
                    self.read_char();
                    self.token_from_pos(TokenKind::Eq)
                }
                _ => self.token_from_pos(TokenKind::Assign),
            },
            '+' => self.token_from_pos(TokenKind::Plus),
            '-' => self.token_from_pos(TokenKind::Minus),
            '!' => match self.peak_char() {
                '=' => {
                    self.read_char();
                    self.token_from_pos(TokenKind::NotEq)
                }
                _ => self.token_from_pos(TokenKind::Bang),
            },
            '*' => self.token_from_pos(TokenKind::Asterisk),
            '/' => self.token_from_pos(TokenKind::Slash),
            '<' => self.token_from_pos(TokenKind::Lt),
            '>' => self.token_from_pos(TokenKind::Gt),
            ';' => self.token_from_pos(TokenKind::Semicolon),
            '(' => self.token_from_pos(TokenKind::Lparen),
            ')' => self.token_from_pos(TokenKind::Rparen),
            ',' => self.token_from_pos(TokenKind::Comma),
            '{' => self.token_from_pos(TokenKind::Lbrace),
            '}' => self.token_from_pos(TokenKind::Rbrace),
            '\0' => Token {
                kind: TokenKind::Eof,
                val: "",
            },
            _ => {
                if ch.is_alphabetic() {
                    let ident = self.read_itentifier();
                    let tok = match ident {
                        "let" => Token {
                            kind: TokenKind::Let,
                            val: ident,
                        },
                        "fn" => Token {
                            kind: TokenKind::Function,
                            val: ident,
                        },
                        "if" => Token {
                            kind: TokenKind::If,
                            val: ident,
                        },
                        "else" => Token {
                            kind: TokenKind::Else,
                            val: ident,
                        },
                        "true" => Token {
                            kind: TokenKind::True,
                            val: ident,
                        },
                        "false" => Token {
                            kind: TokenKind::False,
                            val: ident,
                        },
                        "return" => Token {
                            kind: TokenKind::Return,
                            val: ident,
                        },
                        _ => Token {
                            kind: TokenKind::Ident,
                            val: ident,
                        },
                    };
                    return Some(tok);
                } else if ch.is_numeric() {
                    let n = self.read_number();
                    return Some(Token {
                        kind: TokenKind::Int,
                        val: n,
                    });
                } else {
                    self.token_from_pos(TokenKind::Illegal)
                }
            }
        };
        self.read_char();
        match token.kind {
            TokenKind::Eof => None,
            _ => Some(token),
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}

#[cfg(test)]
mod tests {
    use crate::token::{Token, TokenKind};

    use super::Lexer;

    #[test]
    fn next_token() {
        let input = "let five = 5;
";
        let tokens = vec![
            Token {
                kind: TokenKind::Let,
                val: "let",
            },
            Token {
                kind: TokenKind::Ident,
                val: "five",
            },
            Token {
                kind: TokenKind::Assign,
                val: "=",
            },
            Token {
                kind: TokenKind::Int,
                val: "5",
            },
            Token {
                kind: TokenKind::Semicolon,
                val: ";",
            },
        ];

        let mut lexer = Lexer::new(input);
        for (index, tt) in tokens.into_iter().enumerate() {
            let token = lexer.next_token().unwrap();

            assert_eq!(
                tt, token,
                "index {}, test {:?}, algo {:?}",
                index, tt, token
            );
        }
    }
}
