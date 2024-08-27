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
            '=' => {
                match self.peak_char() {
                    '=' => {
                        self.read_char();
                        Token::EQ
                    },
                    _ => Token::ASSIGN
                }
            },
            '+' => Token::PLUS,
            '-' => Token::MINUS,
            '!' => {
                match self.peak_char() {
                    '=' => {
                        self.read_char();
                        Token::NOT_EQ
                    },
                    _ => Token::BANG
                }
            },
            '*' => Token::ASTERISK,
            '/' => Token::SLASH,
            '<' => Token::LT,
            '>' => Token::GT,
            ';' => Token::SEMICOLON,
            '(' => Token::LPAREN,
            ')' => Token::RPAREN,
            ',' => Token::COMMA,
            '{' => Token::LBRACE,
            '}' => Token::RBRACE,
            '\0' => Token::EOF,
            _ => {
                if ch.is_alphabetic() {
                    let ident = self.read_itentifier();
                    let tok = match ident.as_ref() {
                        "let" => Token::LET,
                        "fn" => Token::FUNCTION,
                        "if" => Token::IF,
                        "else" => Token::ELSE,
                        "true" => Token::TRUE,
                        "false" => Token::FALSE,
                        "return" => Token::RETURN,
                        _ => Token::IDENT(ident),
                    };
                    return tok;
                } else if ch.is_numeric() {
                    let n = self.read_number();
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
        let input = "let five = 5;
let ten = 10;

let add = fn(x, y) {
  x + y;
};

let result = add(five, ten);
!-/*5;
5 < 10 > 5;

if (5 < 10) {
    return true;
} else {
    return false;
}

10 == 10;
10 != 9;
";
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
            Token::BANG,
            Token::MINUS,
            Token::SLASH,
            Token::ASTERISK,
            Token::INT(5),
            Token::SEMICOLON,
            Token::INT(5),
            Token::LT,
            Token::INT(10),
            Token::GT,
            Token::INT(5),
            Token::SEMICOLON,
            Token::IF,
            Token::LPAREN,
            Token::INT(5),
            Token::LT,
            Token::INT(10),
            Token::RPAREN,
            Token::LBRACE,
            Token::RETURN,
            Token::TRUE,
            Token::SEMICOLON,
            Token::RBRACE,
            Token::ELSE,
            Token::LBRACE,
            Token::RETURN,
            Token::FALSE,
            Token::SEMICOLON,
            Token::RBRACE,
            Token::INT(10),
            Token::EQ,
            Token::INT(10),
            Token::SEMICOLON,
            Token::INT(10),
            Token::NOT_EQ,
            Token::INT(9),
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
