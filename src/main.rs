use std::{env::args, fs::read_to_string};

use lexer::Lexer;

mod lexer;
mod token;

fn main() -> Result<(), &'static str> {
    let mut args = args();
    if args.len() < 2 {
        Err("Usage: input file is not provided")
    } else {
        let source = read_to_string(args.nth(1).unwrap()).unwrap();
        let mut lexer = Lexer::new(source);
        let ast = lexer.parse()?;
        for cmd in ast {
            println!("{}", cmd);
        }
        Ok(())
    }
}
