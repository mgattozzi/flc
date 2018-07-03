use std::fmt;

#[derive(Debug)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
    Mod,
    Print,
    PrintLn,
    User(String),
}

impl fmt::Display for Op {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let val = match self {
            Op::Add => "+",
            Op::Sub => "-",
            Op::Div => "/",
            Op::Mul => "*",
            Op::Pow => "^",
            Op::Mod => "%",
            Op::Print => "print",
            Op::PrintLn => "println",
            Op::User(name) => &name,
        };
        write!(f, "{}", val)
    }
}
