use std::{error, io::Read, path::PathBuf, str::FromStr};

mod lexer;
mod tokens;
use lexer::Lexer;

fn main() -> Result<(), Box<dyn error::Error>> {
    let args: Vec<_> = std::env::args().collect();
    let file_path = PathBuf::from_str(&args[0])?;

    run_file(file_path)?;

    Ok(())
}

fn run_file(file: PathBuf) -> Result<(), Box<dyn error::Error>> {
    let mut reader = std::fs::File::open(file)?;
    let mut source = String::new();

    reader.read_to_string(&mut source)?;

    let mut lexer = Lexer::new(&source);
    lexer.scan_tokens();

    dbg!(lexer.tokens());

    Ok(())
}
