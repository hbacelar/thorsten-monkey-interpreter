use std::fmt::{Debug, Display};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Object {
    Integer(i64),
    Boolean(bool),
    ReturnValue(Box<Object>),
    Null,
}

impl Object {
    pub fn is_thruthy(&self) -> bool {
        match self {
            Object::Integer(_) => true,
            Object::Boolean(b) => *b,
            Object::Null => false,
            Object::ReturnValue(obj) => obj.is_thruthy(),
        }
    }
    pub fn type_val(&self) -> &'static str {
        match self {
            Object::Integer(_) => "INTEGER",
            Object::Boolean(_) => "BOOLEAN",
            Object::ReturnValue(_) => "RETURN",
            Object::Null => "NULL",
        }

    }
}

impl Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Object::Integer(n) => write!(f, "{}", n),
            Object::Boolean(b) => {
                if *b {
                    write!(f, "true")
                } else {
                    write!(f, "false")
                }
            }
            Object::Null => write!(f, "null"),
            Object::ReturnValue(obj) => std::fmt::Display::fmt(&obj, f),
        }
    }
}
