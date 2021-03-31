use std::io::{self, stdin, stdout, Write};

fn main() -> io::Result<()> {
    print_prompt()?;

    let mut input = String::new();

    stdin().read_line(&mut input)?;

    println!("Storing '{}'", input.trim());
    Ok(())
}

fn print_prompt() -> io::Result<()> {
    print!("> ");
    stdout().flush()
}
