mod state;

use ast::Ast;
use failure::Error;
use ops::Op;
use prim::Primitive;
use self::state::State;


pub fn evaluate(ast: Ast) -> Result<(), Error> {
    let state = State::instantiate();

    for item in ast.code.into_iter() {
        eval(item, state.clone())?;
    }
    Ok(())
}

fn eval(item: Primitive, st: State) -> Result<Primitive, Error> {
    match item {
        Primitive::Function {
            operation,
            arguments,
        } => {
            match operation {
                Op::Add => {
                    if arguments.len() != 2 {
                        bail!("+ only takes two arguments");
                    }
                    let mut eval_args = Vec::new();
                    for arg in arguments {
                        eval_args.push(eval(arg, st.clone())?);
                    }
                    if let (&Primitive::Number(a), &Primitive::Number(b)) = (&eval_args[0], &eval_args[1]) {
                        Ok(Primitive::Number(a + b))
                    } else {
                        bail!("Unable to evaluate . Arg was not able to evaluate to a number");
                    }
                }
                Op::Div => {
                    if arguments.len() != 2 {
                        bail!("/ only takes two arguments");
                    }
                    let mut eval_args = Vec::new();
                    for arg in arguments {
                        eval_args.push(eval(arg, st.clone())?);
                    }
                    if let (&Primitive::Number(a), &Primitive::Number(b)) = (&eval_args[0], &eval_args[1]) {
                        Ok(Primitive::Number(a + b))
                    } else {
                        bail!("Unable to evaluate . Arg was not able to evaluate to a number");
                    }
                }
                Op::Mul => {
                    if arguments.len() != 2 {
                        bail!("* only takes two arguments");
                    }
                    let mut eval_args = Vec::new();
                    for arg in arguments {
                        eval_args.push(eval(arg, st.clone())?);
                    }
                    if let (&Primitive::Number(a), &Primitive::Number(b)) = (&eval_args[0], &eval_args[1]) {
                        Ok(Primitive::Number(a * b))
                    } else {
                        bail!("Unable to evaluate -. Arg was not able to evaluate to a number");
                    }
                }
                Op::Sub => {
                    if arguments.len() != 2 {
                        bail!("- only takes two arguments");
                    }
                    let mut eval_args = Vec::new();
                    for arg in arguments {
                        eval_args.push(eval(arg, st.clone())?);
                    }
                    if let (&Primitive::Number(a), &Primitive::Number(b)) = (&eval_args[0], &eval_args[1]) {
                        Ok(Primitive::Number(a - b))
                    } else {
                        bail!("Unable to evaluate -. Arg was not able to evaluate to a number");
                    }
                }
                Op::Pow => {
                    if arguments.len() != 2 {
                        bail!("^ only takes two arguments");
                    }
                    let mut eval_args = Vec::new();
                    for arg in arguments {
                        eval_args.push(eval(arg, st.clone())?);
                    }
                    if let (&Primitive::Number(a), &Primitive::Number(b)) = (&eval_args[0], &eval_args[1]) {
                        Ok(Primitive::Number(exponent(a,b)))
                    } else {
                        bail!("Unable to evaluate ^. Arg was not able to evaluate to a number");
                    }
                }
                Op::Mod => {
                    if arguments.len() != 2 {
                        bail!("% only takes two arguments");
                    }
                    let mut eval_args = Vec::new();
                    for arg in arguments {
                        eval_args.push(eval(arg, st.clone())?);
                    }
                    if let (&Primitive::Number(a), &Primitive::Number(b)) = (&eval_args[0], &eval_args[1]) {
                        Ok(Primitive::Number(a % b))
                    } else {
                        bail!("Unable to evaluate %. Arg was not able to evaluate to a number");
                    }
                }
                Op::Print => {
                    let mut eval_args = Vec::new();
                    for arg in arguments {
                        eval_args.push(eval(arg, st.clone())?);
                    }
                    for i in eval_args {
                        print!("{}", i);
                    }
                    Ok(Primitive::AbsoluteUnit)
                }
                Op::PrintLn => {
                    let mut eval_args = Vec::new();
                    for arg in arguments {
                        eval_args.push(eval(arg, st.clone())?);
                    }
                    for i in eval_args {
                        print!("{}", i);
                    }
                    println!();
                    Ok(Primitive::AbsoluteUnit)
                }
                Op::Def => {
                    if arguments.len() != 2 {
                        bail!("define only takes two arguments");
                    }
                    Ok(Primitive::AbsoluteUnit)
                }
                Op::User(_) => bail!("I only support inbuilt right now"),
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
