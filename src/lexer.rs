use crate::token::Token;

pub struct Lexer {
    input: String,
    pub position: usize,
    read_position: usize,
    ch: char,
}

impl Lexer {
    pub fn new(input: String) -> Self {
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

    fn read_itentifier(&mut self) -> &str {
        let pos = self.position;
        while self.ch.is_alphabetic() {
            self.read_char()
        }
        &self.input[pos..self.position]
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

    pub fn next_token(&mut self) -> Option<Token> {
        self.consume_whitespace();

        let ch = self.ch;
        let token = match ch {
            '=' => match self.peak_char() {
                '=' => {
                    self.read_char();
                    Token::Eq
                }
                _ => Token::Assign,
            },
            '+' => Token::Plus,
            '-' => Token::Minus,
            '!' => match self.peak_char() {
                '=' => {
                    self.read_char();
                    Token::NotEq
                }
                _ => Token::Bang,
            },
            '*' => Token::Asterisk,
            '/' => Token::Slash,
            '<' => Token::Lt,
            '>' => Token::Gt,
            ';' => Token::Semicolon,
            '(' => Token::Lparen,
            ')' => Token::Rparen,
            ',' => Token::Comma,
            '{' => Token::Lbrace,
            '}' => Token::Rbrace,
            '\0' => Token::Eof,
            _ => {
                if ch.is_alphabetic() {
                    let ident = self.read_itentifier();
                    let tok = match ident.as_ref() {
                        "let" => Token::Let,
                        "fn" => Token::Function,
                        "if" => Token::If,
                        "else" => Token::Else,
                        "true" => Token::True,
                        "false" => Token::False,
                        "return" => Token::Return,
                        _ => Token::Ident(ident.to_string()),
                    };
                    return Some(tok);
                } else if ch.is_numeric() {
                    let n = self.read_number();
                    return Some(Token::Int(n));
                } else {
                    Token::Illegal
                }
            }
        };
        self.read_char();
        match token {
            Token::Eof => None,
            _ => Some(token),
        }
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
            Token::Let,
            Token::Ident("five".to_string()),
            Token::Assign,
            Token::Int(5),
            Token::Semicolon,
            Token::Let,
            Token::Ident("ten".to_string()),
            Token::Assign,
            Token::Int(10),
            Token::Semicolon,
            Token::Let,
            Token::Ident("add".to_string()),
            Token::Assign,
            Token::Function,
            Token::Lparen,
            Token::Ident("x".to_string()),
            Token::Comma,
            Token::Ident("y".to_string()),
            Token::Rparen,
            Token::Lbrace,
            Token::Ident("x".to_string()),
            Token::Plus,
            Token::Ident("y".to_string()),
            Token::Semicolon,
            Token::Rbrace,
            Token::Semicolon,
            Token::Let,
            Token::Ident("result".to_string()),
            Token::Assign,
            Token::Ident("add".to_string()),
            Token::Lparen,
            Token::Ident("five".to_string()),
            Token::Comma,
            Token::Ident("ten".to_string()),
            Token::Rparen,
            Token::Semicolon,
            Token::Bang,
            Token::Minus,
            Token::Slash,
            Token::Asterisk,
            Token::Int(5),
            Token::Semicolon,
            Token::Int(5),
            Token::Lt,
            Token::Int(10),
            Token::Gt,
            Token::Int(5),
            Token::Semicolon,
            Token::If,
            Token::Lparen,
            Token::Int(5),
            Token::Lt,
            Token::Int(10),
            Token::Rparen,
            Token::Lbrace,
            Token::Return,
            Token::True,
            Token::Semicolon,
            Token::Rbrace,
            Token::Else,
            Token::Lbrace,
            Token::Return,
            Token::False,
            Token::Semicolon,
            Token::Rbrace,
            Token::Int(10),
            Token::Eq,
            Token::Int(10),
            Token::Semicolon,
            Token::Int(10),
            Token::NotEq,
            Token::Int(9),
            Token::Semicolon,
        ];

        let mut lexer = Lexer::new(input.to_string());
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
