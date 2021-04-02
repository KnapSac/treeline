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
                modifiers: KeyModifiers::CONTROL,
                code: KeyCode::Backspace,
            } => {
                // TODO: After support for moving the cursor with the arrow keys is added, this
                //       implementation will most likely fail
                let line = line_buffer.clone();
                let line_parts: Vec<_> = line.rsplitn(2, ' ').collect();
                if line_parts.len() == 2 {
                    // `line_buffer` contained multiple words
                    line_buffer = line_parts.get(1).unwrap().to_string();
                    let chars_to_remove = line_parts.get(0).unwrap().len() + 1;
                    stdout()
                        .queue(cursor::MoveLeft(chars_to_remove as u16))?
                        .queue(terminal::Clear(ClearType::UntilNewLine))?;
                } else {
                    // `line_buffer` contained only 1 word
                    line_buffer.clear();
                    stdout()
                        .queue(cursor::MoveToColumn(0))?
                        .queue(terminal::Clear(ClearType::CurrentLine))?;
                    print_prompt()?;
                }

                stdout().flush()?;
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
