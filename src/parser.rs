use std::mem;

use crate::{
    ast::{
        BlockStatement, Expression, ExpressionStatement, FunctionLiteral, Identifier, IfExpression,
        InfixExpression, IntegerLiteral, LetStatement, Operator, PrefixExpression, Program,
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
    fn prefix_parse(&self, parser: &mut Parser) -> Result<Expression> {
        match &self {
            Token::Ident(ident) => Ok(Expression::Identifier(Identifier {
                value: ident.clone(),
            })),
            Token::Bang | Token::Minus => {
                parser.next_token();

                let right = parser.parse_expression(OperatorPrecedence::Prefix)?;
                Ok(Expression::PrefixExpression(PrefixExpression {
                    right: Box::new(right),
                    operator: self.try_into()?,
                }))
            }
            Token::Int(value) => Ok(Expression::IntegerLiteral(IntegerLiteral { value: *value })),
            Token::True => Ok(Expression::BooleanLiteral(crate::ast::BooleanLiteral {
                value: true,
            })),
            Token::False => Ok(Expression::BooleanLiteral(crate::ast::BooleanLiteral {
                value: false,
            })),
            Token::Lparen => {
                parser.next_token();
                let exp = parser.parse_expression(OperatorPrecedence::Lowest);

                if let Some(Token::Rparen) = parser.peek_token {
                    parser.next_token();
                    exp
                } else {
                    bail!("right parentesis not found after left")
                }
            }
            Token::Function => {
                if let Some(Token::Lparen) = parser.peek_token {
                    parser.next_token();
                } else {
                    bail!("left parentesis not found after if");
                }
                let parameters = parser.parse_func_params()?;

                if let Some(Token::Lbrace) = parser.peek_token {
                    parser.next_token();
                } else {
                    bail!("left brace parentesis not found after if");
                }

                let body = parser.parse_block_statement()?;

                Ok(Expression::FunctionLiteral(FunctionLiteral {
                    body,
                    parameters,
                }))
            }
            Token::If => {
                if let Some(Token::Lparen) = parser.peek_token {
                    parser.next_token();
                } else {
                    bail!("left parentesis not found after if");
                }

                let condition = parser.parse_expression(OperatorPrecedence::Lowest)?;

                // if let Some(Token::Rparen) = parser.peek_token {
                //     parser.next_token();
                // } else {
                //     bail!("right parentesis not found after if");
                // }

                if let Some(Token::Lbrace) = parser.peek_token {
                    parser.next_token();
                } else {
                    bail!("left brace parentesis not found after if");
                }

                let consequence = parser.parse_block_statement()?;

                if let Some(Token::Else) = parser.peek_token {
                    parser.next_token();

                    if let Some(Token::Lbrace) = parser.peek_token {
                        parser.next_token();
                    } else {
                        bail!("left brace parentesis not found after if");
                    }

                    let alternative = parser.parse_block_statement()?;

                    Ok(Expression::IfExpression(IfExpression {
                        condition: Box::new(condition),
                        consequence,
                        alternative: Some(alternative),
                    }))
                } else {
                    Ok(Expression::IfExpression(IfExpression {
                        condition: Box::new(condition),
                        consequence,
                        alternative: None,
                    }))
                }
            }
            _ => bail!("test broken exp {:?}", &self),
        }
    }

    fn infix_parse(&self, left: Expression, parser: &mut Parser) -> Result<Expression> {
        match &self {
            Token::Plus
            | Token::Minus
            | Token::Asterisk
            | Token::Slash
            | Token::Lt
            | Token::Gt
            | Token::Eq
            | Token::NotEq => {
                let op: Operator = self.try_into()?;
                let precedence: OperatorPrecedence = (&op).into();

                parser.next_token();
                parser.next_token();
                let right = parser.parse_expression(precedence)?;
                Ok(Expression::InfixExpression(InfixExpression {
                    right: Box::new(right),
                    left: Box::new(left),
                    operator: op,
                }))
            }
            _ => Ok(left),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum OperatorPrecedence {
    Lowest = 0,
    Equals = 1,
    LessGreater = 2,
    Sum = 3,
    Product = 4,
    Prefix = 5,
    Call = 6,
}

impl From<&Operator> for OperatorPrecedence {
    fn from(value: &Operator) -> Self {
        match value {
            Operator::Minus => Self::Sum,
            Operator::Plus => Self::Sum,
            Operator::Asterisk => Self::Product,
            Operator::Slash => Self::Product,
            Operator::Eq => Self::Equals,
            Operator::NotEq => Self::Equals,
            Operator::Lt => Self::LessGreater,
            Operator::Gt => Self::LessGreater,
            _ => Self::Lowest,
        }
    }
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

                let statement = Ok(Statement::Let(LetStatement {
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
        Ok(Statement::Return(ReturnStatement {
            value: Expression::Identifier(Identifier {
                value: "todo".to_string(),
            }),
        }))
    }

    fn parse_block_statement(&mut self) -> Result<BlockStatement> {
        let mut block_statement = BlockStatement {
            statements: Vec::new(),
        };
        self.next_token();

        while self.current_token.is_some() {
            if let Some(Token::Rbrace) = self.current_token {
                break;
            }
            let stmt = self.parse_statement()?;
            block_statement.statements.push(stmt);
            self.next_token();
        }
        Ok(block_statement)
    }

    fn parse_func_params(&mut self) -> Result<Vec<Identifier>> {
        let mut identifiers = Vec::new();
        self.next_token();

        if let Some(Token::Rparen) = self.current_token {
            self.next_token();
            return Ok(identifiers);
        }

        match &self.current_token {
            Some(Token::Ident(ident)) => identifiers.push(Identifier {
                value: ident.clone(),
            }),
            _ => bail!("identifier not found for parameters"),
        };

        while self.peek_token.is_some() {
            if let Some(Token::Comma) = self.peek_token {
                self.next_token();
                self.next_token();

                match &self.current_token {
                    Some(Token::Ident(ident)) => identifiers.push(Identifier {
                        value: ident.clone(),
                    }),
                    _ => bail!("identifier not found for parameters"),
                };
            } else {
                break;
            }
        }

        if let Some(Token::Rparen) = self.peek_token {
            self.next_token();
            return Ok(identifiers);
        }
        bail!("lparen not found after params");
    }

    fn parse_expression(&mut self, precedence: OperatorPrecedence) -> Result<Expression> {
        if let Some(token) = &self.current_token {
            let curr_token = token.clone();
            let mut left = curr_token.prefix_parse(self)?;

            loop {
                if let Some(Token::Semicolon) = &self.current_token {
                    break;
                }
                if let Some(peak_token) = &self.peek_token {
                    let op: Result<Operator, _> = peak_token.try_into();
                    if op.is_err() {
                        break;
                    }

                    let peak_precedence: OperatorPrecedence = (&op.unwrap()).into();

                    if precedence >= peak_precedence {
                        break;
                    }

                    let curr_peak_token = peak_token.clone();
                    // should it advance the parser here
                    left = curr_peak_token.infix_parse(left, self)?;
                    // should it advance again if it advanced
                } else {
                    break;
                }
            }
            return Ok(left);
        }
        bail!("cannot parse expression");
    }

    fn parse_expression_statement(&mut self) -> Result<Statement> {
        let expression = self.parse_expression(OperatorPrecedence::Lowest)?;

        if let Some(Token::Semicolon) = self.peek_token {
            self.next_token();
        }

        Ok(Statement::Expression(ExpressionStatement { expression }))
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

        while self.current_token.is_some() {
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
    use std::mem;

    use crate::{
        ast::{Expression, Identifier, Operator, Statement},
        lexer::Lexer,
    };

    use super::Parser;

    struct PrefixOperationTests {
        pub input: String,
        pub operator: Operator,
        pub int: i32,
    }

    struct InfixOperationTests {
        pub input: String,
        pub operator: Operator,
        pub left: i32,
        pub right: i32,
    }

    fn test_let_statement(statement: &Statement, val: &str) {
        if let Statement::Let(statement) = statement {
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

    pub fn test_int_literal(exp: &Expression, val: i32) {
        match exp {
            Expression::IntegerLiteral(integer) => {
                assert_eq!(integer.value, val);
            }
            _ => panic!("expression is not integer literal"),
        }
    }

    pub fn test_bool_literal(exp: &Expression, val: bool) {
        match exp {
            Expression::BooleanLiteral(bool) => {
                assert_eq!(bool.value, val);
            }
            _ => panic!("expression is not bool literal"),
        }
    }

    pub fn test_identifier_exp(exp: &Expression, val: String) {
        match exp {
            Expression::Identifier(ident) => {
                assert_eq!(ident.value, val);
            }
            _ => panic!("expression is not identifier"),
        }
    }

    pub fn test_infix_exp(
        exp: &Expression,
        left: &Expression,
        operator: Operator,
        right: &Expression,
    ) {
        match exp {
            Expression::InfixExpression(exp) => {
                assert_eq!(exp.left.as_ref(), left);
                assert_eq!(exp.right.as_ref(), right);
                assert_eq!(exp.operator, operator);
            }
            _ => panic!("expression is not identifier"),
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
                Statement::Return(_) => continue,
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
            Statement::Expression(exp) => {
                test_identifier_exp(&exp.expression, "foobar".to_string())
            }
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
            Statement::Expression(exp) => {
                test_int_literal(&exp.expression, 5);
            }
            _ => panic!("Statment is not identifier expression"),
        }
    }

    #[test]
    fn test_bool_literal_expression() {
        let input = "true;";
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
            Statement::Expression(exp) => {
                test_bool_literal(&exp.expression, true);
            }
            _ => panic!("Statment is not identifier expression"),
        }
    }

    #[test]
    fn test_prefix_expressions() {
        let tests = vec![
            PrefixOperationTests {
                input: "!5".to_string(),
                operator: Operator::Bang,
                int: 5,
            },
            PrefixOperationTests {
                input: "-15".to_string(),
                operator: Operator::Minus,
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

            let stmt = program.statments.get(0).unwrap();

            match stmt {
                Statement::Expression(exp) => match &exp.expression {
                    Expression::PrefixExpression(exp) => {
                        assert_eq!(
                            mem::discriminant(&exp.operator),
                            mem::discriminant(&test.operator)
                        );
                        test_int_literal(&exp.right, test.int);
                    }
                    _ => panic!("Prefix not found expression"),
                },
                _ => panic!("Statment is not identifier expression"),
            }
        }
    }

    #[test]
    fn test_infix_expressions() {
        let tests = vec![
            InfixOperationTests {
                input: "5 + 5;".to_string(),
                operator: Operator::Plus,
                left: 5,
                right: 5,
            },
            InfixOperationTests {
                input: "5 - 5;".to_string(),
                operator: Operator::Minus,
                left: 5,
                right: 5,
            },
            InfixOperationTests {
                input: "5 * 5;".to_string(),
                operator: Operator::Asterisk,
                left: 5,
                right: 5,
            },
            InfixOperationTests {
                input: "5 / 5;".to_string(),
                operator: Operator::Slash,
                left: 5,
                right: 5,
            },
            InfixOperationTests {
                input: "5 > 5;".to_string(),
                operator: Operator::Gt,
                left: 5,
                right: 5,
            },
            InfixOperationTests {
                input: "5 < 5;".to_string(),
                operator: Operator::Lt,
                left: 5,
                right: 5,
            },
            InfixOperationTests {
                input: "5 != 5;".to_string(),
                operator: Operator::NotEq,
                left: 5,
                right: 5,
            },
            InfixOperationTests {
                input: "5 == 5;".to_string(),
                operator: Operator::Eq,
                left: 5,
                right: 5,
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

            let stmt = program.statments.get(0).unwrap();

            match stmt {
                Statement::Expression(exp) => match &exp.expression {
                    Expression::InfixExpression(exp) => {
                        assert_eq!(
                            mem::discriminant(&exp.operator),
                            mem::discriminant(&test.operator)
                        );
                        test_int_literal(&exp.right, test.right);
                        test_int_literal(&exp.left, test.left);
                    }
                    _ => panic!("Prefix not found expression"),
                },
                _ => panic!("Statment is not identifier expression"),
            }
        }
    }

    #[test]
    fn test_if_expression() {
        let input = "if (x < y) { x }";
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
            Statement::Expression(exp) => {
                match &exp.expression {
                    Expression::IfExpression(if_exp) => {
                        test_infix_exp(
                            if_exp.condition.as_ref(),
                            &Expression::Identifier(Identifier {
                                value: "x".to_string(),
                            }),
                            Operator::Lt,
                            &Expression::Identifier(Identifier {
                                value: "y".to_string(),
                            }),
                        );

                        assert_eq!(
                            1,
                            if_exp.consequence.statements.len(),
                            "invalid number of consequence statements: {}",
                            if_exp.consequence.statements.len(),
                        );
                        assert_eq!(None, if_exp.alternative);

                        let stmt = if_exp
                            .consequence
                            .statements
                            .get(0)
                            .expect("invalid condition");

                        match stmt {
                            Statement::Expression(exp) => {
                                test_identifier_exp(&exp.expression, "x".to_string())
                            }
                            _ => panic!("condition statment is not identifier expression"),
                        }
                    }
                    _ => panic!("expression is not if expression"),
                };
            }
            _ => panic!("Statment is not identifier expression"),
        }
    }

    #[test]
    fn test_if_else_expression() {
        let input = "if (x < y) { x } else { y }";
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
            Statement::Expression(exp) => {
                match &exp.expression {
                    Expression::IfExpression(if_exp) => {
                        test_infix_exp(
                            if_exp.condition.as_ref(),
                            &Expression::Identifier(Identifier {
                                value: "x".to_string(),
                            }),
                            Operator::Lt,
                            &Expression::Identifier(Identifier {
                                value: "y".to_string(),
                            }),
                        );

                        assert_eq!(
                            1,
                            if_exp.consequence.statements.len(),
                            "invalid number of consequence statements: {}",
                            if_exp.consequence.statements.len(),
                        );

                        let stmt = if_exp
                            .consequence
                            .statements
                            .get(0)
                            .expect("invalid condition");

                        match stmt {
                            Statement::Expression(exp) => {
                                test_identifier_exp(&exp.expression, "x".to_string())
                            }
                            _ => panic!("condition statment is not identifier expression"),
                        }

                        let alternative = if_exp.alternative.as_ref().unwrap();

                        assert_eq!(
                            1,
                            alternative.statements.len(),
                            "invalid number of alternative statements: {}",
                            alternative.statements.len(),
                        );
                        let stmt = alternative.statements.get(0).expect("invalid condition");

                        match stmt {
                            Statement::Expression(exp) => {
                                test_identifier_exp(&exp.expression, "y".to_string())
                            }
                            _ => panic!("alternative statment is not identifier expression"),
                        }
                    }
                    _ => panic!("expression is not if expression"),
                };
            }
            _ => panic!("Statment is not identifier expression"),
        }
    }

    #[test]
    fn test_function_literal() {
        let input = "fn (x, y) {x + y;}";
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
            Statement::Expression(exp) => match &exp.expression {
                Expression::FunctionLiteral(function) => {
                    assert_eq!(
                        2,
                        function.parameters.len(),
                        "invalid number of parameters: {}",
                        function.parameters.len(),
                    );
                    let param = function.parameters.get(0).unwrap();
                    assert_eq!("x", param.value);
                    let param = function.parameters.get(1).unwrap();
                    assert_eq!("y", param.value);

                    assert_eq!(
                        1,
                        function.body.statements.len(),
                        "invalid number of body statments: {}",
                        function.body.statements.len(),
                    );

                    let stmt = function.body.statements.get(0).expect("invalid body");

                    match stmt {
                        Statement::Expression(exp) => test_infix_exp(
                            &exp.expression,
                            &Expression::Identifier(Identifier {
                                value: "x".to_string(),
                            }),
                            Operator::Plus,
                            &Expression::Identifier(Identifier {
                                value: "y".to_string(),
                            }),
                        ),
                        _ => panic!("body statment is not identifier expression"),
                    }
                }
                _ => panic!("Expression is not a function"),
            },
            _ => panic!("Statment is not identifier expression"),
        }
    }

    #[test]
    fn test_playground() {
        let input = "(-(5 + 5))";
        let lexer = Lexer::new(input.to_string());
        let parser = Parser::new(lexer);

        let program = parser.parse_program().unwrap();

        dbg!(&program);
        assert_eq!(
            1,
            program.statments.len(),
            "invalid number of statements: {}",
            program.statments.len()
        );
    }
}
