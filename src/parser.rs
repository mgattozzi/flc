use std::fs;
use std::fmt;
use std::str::FromStr;
use failure::Error;

#[derive(Debug)]
pub struct Ast {
    pub code: Vec<Type>
}

#[derive(Debug)]
pub enum Type {
    Function {
        operation: Op,
        arguments: Vec<Type>
    },
    Number(i64),
}

#[derive(Debug)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
    User(String)
}

pub fn parse(file: &str) -> Result<Ast, Error> {
    let input = fs::read_to_string(file)?;
    let (_, ast) = parse_input(&input).map_err(|e| format_err!("{}", e))?;
    Ok(ast)
}

named!(parse_input<&str, Ast>,
    do_parse!(
        code: many1!(fn_parse) >>
        (Ast { code })
    )
);

pub fn is_white_space(c: char) -> bool {
    c == ' ' || c == '\r' || c == '\t' || c == '\n'
}

pub fn is_num(c: char) -> bool {
    match c {
        '0' |
        '1' |
        '2' |
        '3' |
        '4' |
        '5' |
        '6' |
        '7' |
        '8' |
        '9' => true,
        _ => false
    }
}

/// This only ever returns a Type::Function
named!(fn_parse<&str, Type>,
    complete!(do_parse!(
        take_while!(is_white_space) >>
        tag!("(") >>
        take_while!(is_white_space) >>
        operation: op_parse >>
        take_while!(is_white_space) >>
        arguments: many0!(number_parse) >>
        tag!(")") >>
        (Type::Function {
            operation,
            arguments
        })
    ))
);

fn str_to_int(input: &str) -> Result<i64, <i64 as FromStr>::Err> {
    input.parse()
}

named!(i64_parse<&str, i64>,
    do_parse!(
        take_while!(is_white_space) >>
        num: map_res!(take_while!(is_num), str_to_int) >>
        (num)
    )
);

named!(number_parse<&str, Type>,
    do_parse!(
        num: i64_parse >>
        (Type::Number(num))
    )
);
named!(op_parse<&str, Op>,
    do_parse!(
        op: ws!(take_until!(" ")) >>
        ({
            match op {
                "+" => Op::Add,
                "-" => Op::Sub,
                "/" => Op::Div,
                "*" => Op::Mul,
                func => Op::User(func.to_string())
            }
        })
    )
);

impl fmt::Display for Op {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let val = match self {
            Op::Add => "+",
            Op::Sub => "-",
            Op::Div => "/",
            Op::Mul => "*",
            Op::User(name) => &name,
        };
        write!(f, "{}", val)
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let val = match self {
            Type::Function { operation, arguments } => {
                let mut out = String::from("(");
                out.push_str(&operation.to_string());
                for i in arguments {
                    out.push_str(&i.to_string());
                }
                out.push(')');
                out
            },
            Type::Number(i64) => i64.to_string(),
        };
        write!(f, "{}", val)
    }
}
