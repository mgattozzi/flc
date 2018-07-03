use ast::Ast;
use failure::Error;
use ops::Op;
use prim::Primitive;
use std::fs;
use std::str::FromStr;

pub fn parse(file: &str) -> Result<Ast, Error> {
    let input = fs::read_to_string(file)?;
    let (_, ast) = parse_input(&input).map_err(|e| format_err!("{}", e))?;
    Ok(ast)
}

named!(parse_input<&str, Ast>,
    do_parse!(
        code: many0!(fn_parse) >>
        (Ast { code })
    )
);

pub fn is_white_space(c: char) -> bool {
    c == ' ' || c == '\r' || c == '\t' || c == '\n'
}

pub fn is_num(c: char) -> bool {
    match c {
        '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => true,
        _ => false,
    }
}

/// This only ever returns a Primitive::Function
named!(fn_parse<&str, Primitive>,
    complete!(do_parse!(
        take_while!(is_white_space) >>
        tag!("(") >>
        take_while!(is_white_space) >>
        operation: op_parse >>
        take_while!(is_white_space) >>
        arguments: many0!(alt!(number_parse | fn_parse | str_parse)) >>
        take_while!(is_white_space) >>
        tag!(")") >>
        (Primitive::Function {
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

named!(number_parse<&str, Primitive>,
    do_parse!(
        num: i64_parse >>
        (Primitive::Number(num))
    )
);

named!(str_parse<&str, Primitive>,
    do_parse!(
        take_while!(is_white_space) >>
        string: delimited!(tag!("\""), take_until!("\""), tag!("\"")) >>
        (Primitive::Str(string.into()))
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
                "%" => Op::Mod,
                "^" => Op::Pow,
                "print" => Op::Print,
                "println" => Op::PrintLn,
                "define" => Op::Def,
                func => Op::User(func.to_string())
            }
        })
    )
);
