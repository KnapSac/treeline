use crossterm::{
    cursor,
    event::{read, Event, KeyCode, KeyEvent, KeyModifiers},
    terminal::{self, ClearType},
    QueueableCommand,
};
use std::io::{self, stdout, Write};
use thiserror::Error;

fn main() {
    let result = run();

    terminal::disable_raw_mode().ok();

    match result {
        Ok(_) => {}
        Err(err) => {
            eprintln!("{}", err);
            std::process::exit(1);
        }
    }
}

fn run() -> Result<()> {
    terminal::enable_raw_mode()?;
    print_prompt()?;

    let mut line_buffer = String::new();
    while let Event::Key(event) = read()? {
        let mut input = None;
        match event {
            KeyEvent {
                modifiers: KeyModifiers::CONTROL,
                code: KeyCode::Char('c'),
            } => {
                return Ok(());
            }
            KeyEvent {
                code: KeyCode::Enter,
                ..
            } => {
                break;
            }
            KeyEvent {
                code: KeyCode::Backspace,
                ..
            } => {
                line_buffer.pop();
                stdout()
                    .queue(cursor::MoveLeft(1))?
                    .queue(terminal::Clear(ClearType::UntilNewLine))?;
                stdout().flush()?;
            }
            KeyEvent {
                code: KeyCode::Char(c),
                ..
            } => {
                line_buffer.push(c);
                input = Some(c);
            }
            _ => {}
        }

        if let Some(c) = input {
            print!("{}", c);
            stdout().flush()?;
        }
    }

    println!();
    println!("{}", line_buffer);

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

    #[error(transparent)]
    CrosstermError(#[from] crossterm::ErrorKind),
}
