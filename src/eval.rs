use ast::Ast;
use failure::Error;
use ops::Op;
use prim::Primitive;

pub fn evaluate(ast: Ast) -> Result<(), Error> {
    for item in ast.code.into_iter() {
        eval(item)?;
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
            match operation {
                Op::Add => {
                    if num_args != 2 {
                        bail!("+ only takes two arguments");
                    }
                    if let (&Primitive::Number(a), &Primitive::Number(b)) = (&eval_args[0], &eval_args[1]) {
                        Ok(Primitive::Number(a + b))
                    } else {
                        bail!("Unable to evaluate . Arg was not able to evaluate to a number");
                    }
                }
                Op::Div => {
                    if num_args != 2 {
                        bail!("/ only takes two arguments");
                    }
                    if let (&Primitive::Number(a), &Primitive::Number(b)) = (&eval_args[0], &eval_args[1]) {
                        Ok(Primitive::Number(a + b))
                    } else {
                        bail!("Unable to evaluate . Arg was not able to evaluate to a number");
                    }
                }
                Op::Mul => {
                    if num_args != 2 {
                        bail!("* only takes two arguments");
                    }
                    if let (&Primitive::Number(a), &Primitive::Number(b)) = (&eval_args[0], &eval_args[1]) {
                        Ok(Primitive::Number(a * b))
                    } else {
                        bail!("Unable to evaluate -. Arg was not able to evaluate to a number");
                    }
                }
                Op::Sub => {
                    if num_args != 2 {
                        bail!("- only takes two arguments");
                    }
                    if let (&Primitive::Number(a), &Primitive::Number(b)) = (&eval_args[0], &eval_args[1]) {
                        Ok(Primitive::Number(a - b))
                    } else {
                        bail!("Unable to evaluate -. Arg was not able to evaluate to a number");
                    }
                }
                Op::Pow => {
                    if num_args != 2 {
                        bail!("^ only takes two arguments");
                    }
                    if let (&Primitive::Number(a), &Primitive::Number(b)) = (&eval_args[0], &eval_args[1]) {
                        Ok(Primitive::Number(exponent(a,b)))
                    } else {
                        bail!("Unable to evaluate ^. Arg was not able to evaluate to a number");
                    }
                }
                Op::Mod => {
                    if num_args != 2 {
                        bail!("% only takes two arguments");
                    }
                    if let (&Primitive::Number(a), &Primitive::Number(b)) = (&eval_args[0], &eval_args[1]) {
                        Ok(Primitive::Number(a % b))
                    } else {
                        bail!("Unable to evaluate %. Arg was not able to evaluate to a number");
                    }
                }
                Op::Print => {
                    for i in eval_args {
                        print!("{}", i);
                    }
                    Ok(Primitive::AbsoluteUnit)
                }
                Op::PrintLn => {
                    for i in eval_args {
                        print!("{}", i);
                    }
                    println!();
                    Ok(Primitive::AbsoluteUnit)
                }
                _ => bail!("I only support + - / % ^and * right now"),
            }
        }
        num @ Primitive::Number(_) => Ok(num),
        string @ Primitive::Str(_) => Ok(string),
        abs @ Primitive::AbsoluteUnit => Ok(abs),
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
