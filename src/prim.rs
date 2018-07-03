use ops::Op;
use std::fmt;

#[derive(Debug)]
pub enum Primitive {
    Function {
        operation: Op,
        arguments: Vec<Primitive>,
    },
    Number(i64),
    AbsoluteUnit
}

impl fmt::Display for Primitive {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let val = match self {
            Primitive::Function {
                operation,
                arguments,
            } => {
                let mut out = String::from("(");
                out.push_str(&operation.to_string());
                for i in arguments {
                    out.push(' ');
                    out.push_str(&i.to_string());
                }
                out.push(')');
                out
            }
            Primitive::Number(i64) => i64.to_string(),
            Primitive::AbsoluteUnit => "()".to_string(),
        };
        write!(f, "{}", val)
    }
}
