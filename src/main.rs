#[macro_use]
extern crate failure;
#[macro_use]
extern crate nom;
extern crate im;

pub mod ast;
pub mod eval;
pub mod ops;
pub mod parser;
pub mod prim;
pub mod special_form;

use failure::Error;

fn main() {
    if let Err(e) = run() {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}

fn run() -> Result<(), Error> {
    let ast = parser::parse("test.fl")?;
    eval::evaluate(ast)?;
    Ok(())
}
