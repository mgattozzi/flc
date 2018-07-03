use ast::Ast;
use failure::Error;
use ops::Op;
use prim::Primitive;

pub fn evaluate(ast: Ast) -> Result<(), Error> {
    for item in ast.code {
        match item {
            Primitive::Function {
                operation,
                arguments,
            } => {
                if arguments.len() != 2 {
                    bail!("I only support + - / and * right now");
                }
                if let (&Primitive::Number(ref a), &Primitive::Number(ref b)) =
                    (&arguments[0], &arguments[1])
                {
                    let value = match operation {
                        Op::Add => a + b,
                        Op::Div => a / b,
                        Op::Mul => a * b,
                        Op::Sub => a - b,
                        _ => bail!("I only support + - / and * right now"),
                    };

                    println!("{} {} {} = {}", a, operation, b, value);
                } else {
                    bail!("I only support numbers in function args right now");
                }
            }
            Primitive::Number(_) => bail!("A number on it's own is invalid"),
        }
    }

    Ok(())
}
