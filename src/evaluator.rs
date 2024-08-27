use crate::{
    ast::{Expression, Node, Program, Statement},
    object::Object,
};
use anyhow::{bail, Result};

pub struct Evaluator {}

impl Evaluator {
    fn inner_eval(node: &Node) -> Result<Object> {
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
            Expression::BooleanLiteral(_) => todo!(),
            Expression::Prefix(_) => todo!(),
            Expression::Infix(_) => todo!(),
            Expression::If(_) => todo!(),
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
        Self::inner_eval(&Node::Program(program))
    }
}

#[cfg(test)]
mod tests {
    use core::panic;

    use anyhow::Result;

    use crate::{lexer::Lexer, object::Object, parser::Parser};

    use super::Evaluator;

    struct ObjectTest<'a> {
        pub input: &'a str,
        pub expected: Object,
    }

    // fn test_integer_object(obj: &Object, expected: i64) {
    //     if let Object::Integer(val) = obj {
    //         assert_eq!(*val, expected, "invalid integer value");
    //     }
    //     panic!("obj is not integer");
    // }

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
