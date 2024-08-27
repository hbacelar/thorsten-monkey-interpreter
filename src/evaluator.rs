use crate::{
    ast::{Expression, Node, Operator, Program, Statement},
    object::Object,
};
use anyhow::{bail, Result};

pub struct Evaluator {}

impl Evaluator {
    fn eval_node(node: &Node) -> Result<Object> {
        match node {
            Node::Program(program) => Self::eval_statments(&program.statments),
            Node::Expression(exp) => Self::eval_exp(exp),
            Node::Statement(stmt) => Self::eval_statment(stmt),
        }
    }

    fn eval_exp(exp: &Expression) -> Result<Object> {
        match exp {
            Expression::Callable(_) => todo!(),
            Expression::IntegerLiteral(int) => Ok(Object::Integer(int.value)),
            Expression::BooleanLiteral(b) => Ok(Object::Boolean(b.value)),
            Expression::Prefix(exp) => {
                let right = Self::eval_exp(&exp.right)?;

                match exp.operator {
                    Operator::Bang => match right {
                        Object::Boolean(true) => Ok(Object::Boolean(false)),
                        Object::Boolean(false) => Ok(Object::Boolean(true)),
                        Object::Null => Ok(Object::Boolean(true)),
                        _ => Ok(Object::Boolean(false)),
                    },
                    Operator::Minus => {
                        if let Object::Integer(i) = right {
                            return Ok(Object::Integer(-i));
                        }
                        Ok(Object::Null)
                    }
                    _ => Ok(Object::Null),
                }
            }
            Expression::Infix(exp) => {
                // test for int operators
                if let Object::Integer(lval) = Self::eval_exp(&exp.left)? {
                    if let Object::Integer(rval) = Self::eval_exp(&exp.right)? {
                        return match exp.operator {
                            // int result
                            Operator::Minus => Ok(Object::Integer(lval - rval)),
                            Operator::Plus => Ok(Object::Integer(lval + rval)),
                            Operator::Asterisk => Ok(Object::Integer(lval * rval)),
                            Operator::Slash => Ok(Object::Integer(lval / rval)),
                            Operator::Eq => Ok(Object::Boolean(lval == rval)),
                            Operator::NotEq => Ok(Object::Boolean(lval != rval)),
                            Operator::Lt => Ok(Object::Boolean(lval < rval)),
                            Operator::Gt => Ok(Object::Boolean(lval > rval)),
                            _ => Ok(Object::Boolean(false)),
                        };
                    }
                }

                // Test for all operators
                match exp.operator {
                    Operator::Eq => Ok(Object::Boolean(exp.left == exp.right)),
                    Operator::NotEq => Ok(Object::Boolean(exp.left != exp.right)),
                    _ => Ok(Object::Null),
                }
            }
            Expression::If(exp) => {
                let condition = Self::eval_exp(&exp.condition)?;
                if condition.is_thruthy() {
                    Self::eval_statments(&exp.consequence.statements)
                } else if let Some(alternative) = &exp.alternative {
                    Self::eval_statments(&alternative.statements)
                } else {
                    Ok(Object::Null)
                }

            },
            Expression::Call(_) => todo!(),
        }
    }

    fn eval_statment(stmt: &Statement) -> Result<Object> {
        match stmt {
            Statement::Expression(exp) => Self::eval_exp(&exp.expression),
            Statement::Block(_) => todo!(),
            Statement::Let(_) => todo!(),
            Statement::Return(_) => todo!(),
        }
    }

    fn eval_statments(stmts: &Vec<Statement>) -> Result<Object> {
        let mut obj = None;

        for stmt in stmts {
            obj = Some(Self::eval_statment(stmt)?);
        }

        if let Some(obj) = obj {
            return Ok(obj);
        }
        bail!("empty statments");
    }

    pub fn eval(program: Program) -> Result<Object> {
        Self::eval_node(&Node::Program(program))
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Result;

    use crate::{lexer::Lexer, object::Object, parser::Parser};

    use super::Evaluator;

    struct ObjectTest<'a> {
        pub input: &'a str,
        pub expected: Object,
    }

    fn test_eval(input: &str) -> Result<Object> {
        let l = Lexer::new(input);
        let p = Parser::new(l);
        let program = p.parse_program();

        Evaluator::eval(program)
    }

    #[test]
    fn test_eval_integer_expression() {
        let tests = vec![
            ObjectTest {
                input: "5",
                expected: Object::Integer(5),
            },
            ObjectTest {
                input: "10",
                expected: Object::Integer(10),
            },
            ObjectTest {
                input: "-5",
                expected: Object::Integer(-5),
            },
            ObjectTest {
                input: "-10",
                expected: Object::Integer(-10),
            },
            ObjectTest {
                input: "5 + 5 + 5 + 5 - 10",
                expected: Object::Integer(10),
            },
            ObjectTest {
                input: "2 * 2 * 2 * 2 * 2",
                expected: Object::Integer(32),
            },
            ObjectTest {
                input: "5 + 2 * 10",
                expected: Object::Integer(25),
            },
            ObjectTest {
                input: "50 / 2 * 2 + 10",
                expected: Object::Integer(60),
            },
            ObjectTest {
                input: "(5 + 10 * 2 + 15 / 3) * 2 + -10",
                expected: Object::Integer(50),
            },
        ];

        for test in tests {
            let obj = test_eval(test.input).unwrap();
            assert_eq!(
                obj, test.expected,
                "object doesnt match expected: {:?}, {:?}",
                obj, test.expected
            );
        }
    }

    #[test]
    fn test_eval_bool_expression() {
        let tests = vec![
            ObjectTest {
                input: "true",
                expected: Object::Boolean(true),
            },
            ObjectTest {
                input: "false",
                expected: Object::Boolean(false),
            },
            ObjectTest {
                input: "5 > 3",
                expected: Object::Boolean(true),
            },
            ObjectTest {
                input: "5 < 3",
                expected: Object::Boolean(false),
            },
            ObjectTest {
                input: "1 == 1",
                expected: Object::Boolean(true),
            },
            ObjectTest {
                input: "1 != 1",
                expected: Object::Boolean(false),
            },
            ObjectTest {
                input: "true != true",
                expected: Object::Boolean(false),
            },
            ObjectTest {
                input: "true == true",
                expected: Object::Boolean(true),
            },
            ObjectTest {
                input: "false != false",
                expected: Object::Boolean(false),
            },
            ObjectTest {
                input: "false == false",
                expected: Object::Boolean(true),
            },
        ];

        for test in tests {
            let obj = test_eval(test.input).unwrap();
            assert_eq!(
                obj, test.expected,
                "object doesnt match expected: {:?}, {:?}",
                obj, test.expected
            );
        }
    }

    #[test]
    fn test_eval_bang_operator() {
        let tests = vec![
            ObjectTest {
                input: "!true",
                expected: Object::Boolean(false),
            },
            ObjectTest {
                input: "!false",
                expected: Object::Boolean(true),
            },
            ObjectTest {
                input: "!5",
                expected: Object::Boolean(false),
            },
            ObjectTest {
                input: "!!false",
                expected: Object::Boolean(false),
            },
            ObjectTest {
                input: "!!false",
                expected: Object::Boolean(false),
            },
            ObjectTest {
                input: "!!5",
                expected: Object::Boolean(true),
            },
        ];

        for test in tests {
            let obj = test_eval(test.input).unwrap();
            assert_eq!(
                obj, test.expected,
                "object doesnt match expected: {:?}, {:?}",
                obj, test.expected
            );
        }
    }
    
    #[test]
    fn test_eval_if_else_expressions() {
        let tests = vec![
            ObjectTest {
                input: "if (true) { 10 }",
                expected: Object::Integer(10),
            },
            ObjectTest {
                input: "if (false) { 10 }",
                expected: Object::Null,
            },
            ObjectTest {
                input: "if (1) { 10 }",
                expected: Object::Integer(10),
            },
            ObjectTest {
                input: "if (1 < 2) { 10 }",
                expected: Object::Integer(10),
            },
            ObjectTest {
                input: "if (1 > 2) { 10 } else { 20 }",
                expected: Object::Integer(20),
            },
            ObjectTest {
                input: "if (1 < 2) { 10 } else { 20 }",
                expected: Object::Integer(10),
            },
        ];

        for test in tests {
            let obj = test_eval(test.input).unwrap();
            assert_eq!(
                obj, test.expected,
                "object doesnt match expected: {:?}, {:?}",
                obj, test.expected
            );
        }
    }
}
