use crate::{
    ast::{BlockStatement, Expression, Node, Operator, Program, Statement},
    object::Object,
};
use anyhow::{bail, Result};

pub struct Evaluator {}

impl Evaluator {
    fn eval_node(node: &Node) -> Result<Object> {
        match node {
            Node::Program(program) => Self::eval_program(&program),
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
                        bail!(
                            "unknown operator: -{}",
                            right.type_val(),
                        );
                    }
                    // TODO: check
                    _ => Ok(Object::Null),
                }
            }
            Expression::Infix(exp) => {
                let left_eval = Self::eval_exp(&exp.left)?;
                let right_eval = Self::eval_exp(&exp.right)?;

                // test for int operators
                if let Object::Integer(lval) = left_eval {
                    if let Object::Integer(rval) = right_eval {
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
                            _ => bail!(
                                "unknown operator: {} {} {}",
                                left_eval.type_val(),
                                exp.operator,
                                right_eval.type_val()
                            ),
                        };
                    }
                }

                // Test for all operators
                match &exp.operator {
                    Operator::Eq => Ok(Object::Boolean(left_eval == right_eval)),
                    Operator::NotEq => Ok(Object::Boolean(left_eval != right_eval)),
                    op => {
                        if left_eval.type_val() == right_eval.type_val() {
                            bail!(
                                "unknown operator: {} {} {}",
                                left_eval.type_val(),
                                op,
                                right_eval.type_val()
                            )
                        }
                        bail!(
                            "type mismatch: {} {} {}",
                            left_eval.type_val(),
                            op,
                            right_eval.type_val()
                        )
                    }
                }
            }
            Expression::If(exp) => {
                let condition = Self::eval_exp(&exp.condition)?;
                if condition.is_thruthy() {
                    Self::eval_block_statments(&exp.consequence)
                } else if let Some(alternative) = &exp.alternative {
                    Self::eval_block_statments(&alternative)
                } else {
                    Ok(Object::Null)
                }
            }
            Expression::Call(_) => todo!(),
        }
    }

    fn eval_statment(stmt: &Statement) -> Result<Object> {
        match stmt {
            Statement::Expression(exp) => Self::eval_exp(&exp.expression),
            Statement::Return(r) => {
                let val = Self::eval_exp(&r.value)?;
                Ok(Object::ReturnValue(Box::new(val)))
            }
            Statement::Block(_) => todo!(),
            Statement::Let(_) => todo!(),
        }
    }

    fn eval_program(program: &Program) -> Result<Object> {
        let mut obj = None;

        for stmt in &program.statments {
            obj = Some(Self::eval_statment(stmt)?);
            if let Some(Object::ReturnValue(r)) = obj {
                return Ok(*r);
            }
        }

        if let Some(obj) = obj {
            return Ok(obj);
        }
        bail!("empty statments");
    }

    fn eval_block_statments(block: &BlockStatement) -> Result<Object> {
        let mut obj = None;

        for stmt in &block.statements {
            obj = Some(Self::eval_statment(stmt)?);
            if let Some(Object::ReturnValue(_)) = obj {
                return Ok(obj.unwrap());
            }
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

    struct ErrorTest<'a> {
        pub input: &'a str,
        pub expected: &'a str,
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

    #[test]
    fn test_return_statments() {
        let tests = vec![
            ObjectTest {
                input: "return 10;",
                expected: Object::Integer(10),
            },
            ObjectTest {
                input: "return 10; 9;",
                expected: Object::Integer(10),
            },
            ObjectTest {
                input: "return 2 * 5; 9;",
                expected: Object::Integer(10),
            },
            ObjectTest {
                input: "9; return 2 * 5; 6;",
                expected: Object::Integer(10),
            },
            ObjectTest {
                input: "if (10 > 1) { if (10 > 1) {return 10;} return 1;}",
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

    #[test]
    fn test_errors() {
        let tests = vec![
            ErrorTest {
                input: "5 + true;",
                expected: "type mismatch: INTEGER + BOOLEAN",
            },
            ErrorTest {
                input: "5 + true; 5;",
                expected: "type mismatch: INTEGER + BOOLEAN",
            },
            ErrorTest {
                input: "-true;",
                expected: "unknown operator: -BOOLEAN",
            },
            ErrorTest {
                input: "true + true;",
                expected: "unknown operator: BOOLEAN + BOOLEAN",
            },
            ErrorTest {
                input: "true + true; 5;",
                expected: "unknown operator: BOOLEAN + BOOLEAN",
            },
            ErrorTest {
                input: "if (10 > 1) { if (10 > 1) {return true + false;} return 1;}",
                expected: "unknown operator: BOOLEAN + BOOLEAN",
            },
            ErrorTest {
                input: "if (10 > 1) { return true + false;}",
                expected: "unknown operator: BOOLEAN + BOOLEAN",
            },
        ];

        for test in tests {
            let obj = test_eval(test.input).unwrap_err();
            dbg!(&test.input);
            assert_eq!(
                obj.to_string(),
                test.expected,
                "object doesnt match expected: {:?}, {:?}",
                obj.to_string(),
                test.expected
            );
        }
    }
}
