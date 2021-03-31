use std::io::{self, stdin, stdout, Write};
use thiserror::Error;

fn main() -> Result<()> {
    print_prompt()?;

    let mut input = String::new();

    stdin().read_line(&mut input)?;

    println!("Storing '{}'", input.trim());
    Ok(())
}

fn print_prompt() -> Result<()> {
    print!("> ");
    stdout().flush()?;

    Ok(())
}

type Result<R, E = Error> = std::result::Result<R, E>;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    IoError(#[from] io::Error),
}
