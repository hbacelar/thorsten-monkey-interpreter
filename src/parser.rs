use std::mem;

use crate::{
    ast::{Expression, Identifier, LetStatement, Program, ReturnStatement, Statement},
    lexer::Lexer,
    token::Token,
};
use anyhow::{bail, Result};

pub struct Parser {
    lexer: Lexer,
    current_token: Option<Token>,
    peek_token: Option<Token>,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Parser {
        let mut p = Parser {
            lexer,
            current_token: None,
            peek_token: None,
        };
        p.next_token();
        p.next_token();
        p
    }

    fn next_token(&mut self) {
        self.current_token = self.peek_token.take();
        self.peek_token = self.lexer.next_token();
    }

    fn parse_let_statement(&mut self) -> Result<Statement> {
        if let Some(Token::Ident(_)) = &mut self.peek_token {
            self.next_token();

            let ident = match &mut self.current_token {
                Some(Token::Ident(val)) => val,
                _ => unreachable!("already validated through peek"),
            };
            let value = mem::take(ident);
            let name = Identifier { value };

            if let Some(Token::Assign) = self.peek_token {
                self.next_token();

                // TODO continue
                loop {
                    self.next_token();
                    if let Some(Token::Semicolon) = self.current_token {
                        break;
                    }
                }

                let statement = Ok(Statement::LetStatement(LetStatement {
                    name,
                    value: Expression::Identifier(Identifier {
                        value: "todo".to_string(),
                    }),
                }));
                return statement;
            }
        };
        bail!("expected token to be ident got: {:?}", self.peek_token);
    }

    fn parse_return_statement(&mut self) -> Result<Statement> {
        // TODO continue
        loop {
            self.next_token();
            if let Some(Token::Semicolon) = self.current_token {
                break;
            }
        }
        let statement = Ok(Statement::ReturnStatement(ReturnStatement {
            value: Expression::Identifier(Identifier {
                value: "todo".to_string(),
            }),
        }));
        return statement;
    }

    fn parse_statement(&mut self) -> Result<Statement> {
        match self.current_token {
            Some(Token::Let) => self.parse_let_statement(),
            Some(Token::Return) => self.parse_return_statement(),
            _ => todo!(),
        }
    }

    pub fn parse_program(mut self) -> Result<Program> {
        let mut p = Program::new();

        while let Some(_) = self.current_token {
            match self.parse_statement() {
                Ok(stmt) => p.statments.push(stmt),
                Err(err) => p.errors.push(err.to_string()),
            }
            self.next_token();
        }
        Ok(p)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        ast::{Identifier, Statement},
        lexer::Lexer,
    };

    use super::Parser;

    fn test_let_statement(statement: &Statement, val: &str) {
        if let Statement::LetStatement(statement) = statement {
            assert_eq!(
                statement.name.value.as_str(),
                val,
                "name value do not match: {}, {}",
                statement.name.value.as_str(),
                val
            );
        } else {
            assert!(false, "statment was not let: {:?}", statement);
        }
    }

    #[test]
    fn test_return_statements() {
        let input = "
return 5;
return 10;
return 838383;
";
        let lexer = Lexer::new(input.to_string());
        let parser = Parser::new(lexer);

        let program = parser.parse_program().unwrap();

        assert_eq!(
            3,
            program.statments.len(),
            "invalid number of statements: {}",
            program.statments.len()
        );

        for stmt in program.statments {
            match stmt {
                Statement::ReturnStatement(_) => continue,
                _ => {
                    panic!("Not found return statement")
                }
            };
        }
    }

    #[test]
    fn test_let_statements() {
        let input = "
let x = 5;
let y = 10;
let foobar = 838383;
";
        let lexer = Lexer::new(input.to_string());
        let parser = Parser::new(lexer);

        let program = parser.parse_program().unwrap();

        assert_eq!(
            3,
            program.statments.len(),
            "invalid number of statements: {}",
            program.statments.len()
        );

        let tests = vec![
            Identifier {
                value: "x".to_string(),
            },
            Identifier {
                value: "y".to_string(),
            },
            Identifier {
                value: "foobar".to_string(),
            },
        ];

        for (i, ident) in tests.iter().enumerate() {
            let stmt = program.statments.get(i).unwrap();
            test_let_statement(stmt, &ident.value);
        }
    }
}
