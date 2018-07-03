use ast::Ast;
use failure::Error;
use ops::Op;
use prim::Primitive;

pub fn evaluate(ast: Ast) -> Result<(), Error> {
    for item in ast.code.into_iter() {
        let fn_str = item.to_string();
        let i = eval(item)?;
        println!("{} = {}", fn_str, i);
    }
    Ok(())
}

fn eval(item: Primitive) -> Result<Primitive, Error> {
    match item {
        Primitive::Function {
            operation,
            arguments,
        } => {
            let num_args = arguments.len();
            let mut eval_args = Vec::new();
            for arg in arguments {
                eval_args.push(eval(arg)?);
            }
            if let (&Primitive::Number(a), &Primitive::Number(b)) = (&eval_args[0], &eval_args[1]) {
                let value = match operation {
                    Op::Add => {
                        if num_args != 2 {
                            bail!("+ only takes two arguments");
                        }
                        a + b
                    }
                    Op::Div => {
                        if num_args != 2 {
                            bail!("/ only takes two arguments");
                        }
                        a / b
                    }
                    Op::Mul => {
                        if num_args != 2 {
                            bail!("* only takes two arguments");
                        }
                        a * b
                    }
                    Op::Sub => {
                        if num_args != 2 {
                            bail!("- only takes two arguments");
                        }
                        a - b
                    }
                    Op::Pow => {
                        if num_args != 2 {
                            bail!("^ only takes two arguments");
                        }
                        exponent(a, b)
                    }
                    Op::Mod => {
                        if num_args != 2 {
                            bail!("% only takes two arguments");
                        }
                        a % b
                    }
                    _ => bail!("I only support + - / % ^and * right now"),
                };
                Ok(Primitive::Number(value))
            } else {
                bail!("I only support numbers in function args right now");
            }
        }
        num @ Primitive::Number(_) => Ok(num),
    }
}

// Overflow what's that? Seriously though this is pretty fragile
fn exponent(x: i64, n: i64) -> i64 {
    // Will need to handle numbers better at some point
    if n <= 0 {
        return 1;
    }

    let mut store = x;
    for _ in 1..n {
        store *= x;
    }
    store
}
