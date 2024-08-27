use crate::token::Token;

pub struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    ch: char,
}

impl Lexer {
    fn new(input: String) -> Self {
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

    fn read_itentifier(&mut self) -> String {
        let pos = self.position;
        while self.ch.is_alphabetic() {
            self.read_char()
        }
        self.input[pos..self.position].to_string()
    }

    fn read_number(&mut self) -> i32 {
        let pos = self.position;
        while self.ch.is_numeric() {
            self.read_char()
        }
        self.input[pos..self.position]
            .parse()
            .expect("already validated with is numberic")
    }

    fn consume_whitespace(&mut self) {
        while self.ch == ' ' || self.ch == '\t' || self.ch == '\n' || self.ch == '\r' {
            self.read_char();
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.consume_whitespace();

        let ch = self.ch;
        let token = match ch {
            '=' => Token::ASSIGN,
            ';' => Token::SEMICOLON,
            '(' => Token::LPAREN,
            ')' => Token::RPAREN,
            ',' => Token::COMMA,
            '+' => Token::PLUS,
            '{' => Token::LBRACE,
            '}' => Token::RBRACE,
            '\0' => Token::EOF,
            _ => {
                if ch.is_alphabetic() {
                    let ident = self.read_itentifier();
                    let tok = match ident.as_ref() {
                        "let" => Token::LET,
                        "fn" => Token::FUNCTION,
                        _ => Token::IDENT(ident),
                    };
                    // return because read_itendifier advanced the read_char
                    return tok;
                } else if ch.is_numeric() {
                    let n = self.read_number();
                    // return because read_itendifier advanced the read_char
                    return Token::INT(n);
                } else {
                    Token::ILLEGAL
                }
            }
        };
        self.read_char();
        token
    }
}

#[cfg(test)]
mod tests {
    use crate::token::Token;

    use super::Lexer;

    #[test]
    fn next_token() {
        let input = "let five = 5;\nlet ten = 10;\n\nlet add = fn(x,y) {\n x + y;\n};\nlet result = add(five, ten);\n";
        let tokens = vec![
            Token::LET,
            Token::IDENT("five".to_string()),
            Token::ASSIGN,
            Token::INT(5),
            Token::SEMICOLON,
            Token::LET,
            Token::IDENT("ten".to_string()),
            Token::ASSIGN,
            Token::INT(10),
            Token::SEMICOLON,
            Token::LET,
            Token::IDENT("add".to_string()),
            Token::ASSIGN,
            Token::FUNCTION,
            Token::LPAREN,
            Token::IDENT("x".to_string()),
            Token::COMMA,
            Token::IDENT("y".to_string()),
            Token::RPAREN,
            Token::LBRACE,
            Token::IDENT("x".to_string()),
            Token::PLUS,
            Token::IDENT("y".to_string()),
            Token::SEMICOLON,
            Token::RBRACE,
            Token::SEMICOLON,
            Token::LET,
            Token::IDENT("result".to_string()),
            Token::ASSIGN,
            Token::IDENT("add".to_string()),
            Token::LPAREN,
            Token::IDENT("five".to_string()),
            Token::COMMA,
            Token::IDENT("ten".to_string()),
            Token::RPAREN,
            Token::SEMICOLON,
        ];

        let mut lexer = Lexer::new(input.to_string());

        for (index, tt) in tokens.into_iter().enumerate() {
            let token = lexer.next_token();
            assert_eq!(
                tt, token,
                "index {}, test {:?}, algo {:?}",
                index, tt, token
            );
        }
    }
}
