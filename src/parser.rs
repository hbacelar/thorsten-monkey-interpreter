use std::mem;

use crate::{
    ast::{
        Expression, ExpressionStatement, Identifier, IntegerLiteral, LetStatement, Program,
        ReturnStatement, Statement,
    },
    lexer::Lexer,
    token::Token,
};
use anyhow::{bail, Result};

pub struct Parser {
    lexer: Lexer,
    current_token: Option<Token>,
    peek_token: Option<Token>,
}

impl Token {
    fn prefix_parse(&self) -> Option<Expression> {
        match &self {
            Token::Ident(ident) => Some(Expression::Identifier(Identifier {
                value: ident.clone(),
            })),
            Token::Int(value) => Some(Expression::IntegerLiteral(IntegerLiteral { value: *value })),
            _ => None,
        }
    }

    fn infix_parse(&self, exp: &Expression) -> Option<Expression> {
        None
    }
}

#[derive(Debug, PartialEq, Eq)]
enum OperatorPrecedence {
    Lowest = 0,
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

    fn parse_expression(&self, _precedence: OperatorPrecedence) -> Result<Expression> {
        if let Some(token) = &self.current_token {
            if let Some(expression) = token.prefix_parse() {
                return Ok(expression);
            }
        }
        bail!("cannot parse expression");
    }

    fn parse_expression_statement(&mut self) -> Result<Statement> {
        let expression = self.parse_expression(OperatorPrecedence::Lowest)?;

        if let Some(Token::Semicolon) = self.peek_token {
            self.next_token();
        }

        Ok(Statement::ExpressionStatement(ExpressionStatement {
            expression,
        }))
    }

    fn parse_statement(&mut self) -> Result<Statement> {
        match self.current_token {
            Some(Token::Let) => self.parse_let_statement(),
            Some(Token::Return) => self.parse_return_statement(),
            _ => self.parse_expression_statement(),
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
    use core::panic;

    use crate::{
        ast::{Expression, Identifier, Statement},
        lexer::Lexer,
    };

    use super::Parser;

    struct PrefixOperationTests {
        pub input: String,
        pub operator: String,
        pub int: i32,
    }

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

    #[test]
    fn test_identifier_expression() {
        let input = "foobar;";
        let lexer = Lexer::new(input.to_string());
        let parser = Parser::new(lexer);

        let program = parser.parse_program().unwrap();

        assert_eq!(
            1,
            program.statments.len(),
            "invalid number of statements: {}",
            program.statments.len()
        );

        let stmt = program.statments.get(0).unwrap();

        match stmt {
            Statement::ExpressionStatement(exp) => match &exp.expression {
                Expression::Identifier(ident) => {
                    assert_eq!(ident.value, "foobar".to_string());
                }
                _ => panic!("Statment is not identifier expression"),
            },
            _ => panic!("Statment is not identifier expression"),
        }
    }

    #[test]
    fn test_integer_literal_expression() {
        let input = "5;";
        let lexer = Lexer::new(input.to_string());
        let parser = Parser::new(lexer);

        let program = parser.parse_program().unwrap();

        assert_eq!(
            1,
            program.statments.len(),
            "invalid number of statements: {}",
            program.statments.len()
        );

        let stmt = program.statments.get(0).unwrap();

        match stmt {
            Statement::ExpressionStatement(exp) => match &exp.expression {
                Expression::IntegerLiteral(integer) => {
                    assert_eq!(integer.value, 5);
                }
                _ => panic!("Statment is not identifier expression"),
            },
            _ => panic!("Statment is not identifier expression"),
        }
    }

    #[test]
    fn test_prefix_expressions() {
        let tests = vec![
            PrefixOperationTests {
                input: "!5".to_string(),
                operator: "!".to_string(),
                int: 5,
            },
            PrefixOperationTests {
                input: "-15".to_string(),
                operator: "-".to_string(),
                int: 15,
            },
        ];

        for test in tests {
            let lexer = Lexer::new(test.input);
            let parser = Parser::new(lexer);

            let program = parser.parse_program().unwrap();

            assert_eq!(
                1,
                program.statments.len(),
                "invalid number of statements: {}",
                program.statments.len()
            );
        }

        // let input = "5;";
        // let lexer = Lexer::new(input.to_string());
        // let parser = Parser::new(lexer);
        //
        // let program = parser.parse_program().unwrap();
        //
        // assert_eq!(
        //     1,
        //     program.statments.len(),
        //     "invalid number of statements: {}",
        //     program.statments.len()
        // );
        //
        // let stmt = program.statments.get(0).unwrap();
        //
        // match stmt {
        //     Statement::ExpressionStatement(exp) => match &exp.expression {
        //         Expression::IntegerLiteral(integer) => {
        //             assert_eq!(integer.value, 5);
        //         }
        //         _ => panic!("Statment is not identifier expression"),
        //     },
        //     _ => panic!("Statment is not identifier expression"),
        // }
    }
}
