use crossterm::{
    cursor,
    event::{read, Event, KeyCode, KeyEvent, KeyModifiers},
    style::{Colorize, Print},
    terminal::{self, ClearType},
    QueueableCommand,
};
use std::{
    io::{self, stdout, Write},
    process,
};
use thiserror::Error;
use treeline::Trie;

fn main() {
    let result = run();

    terminal::disable_raw_mode().ok();

    match result {
        Ok(_) => {}
        Err(err) => {
            eprintln!("{}", err);
            process::exit(1);
        }
    }
}

fn run() -> Result<()> {
    terminal::enable_raw_mode()?;

    let mut history = Trie::new();
    loop {
        let input = get_input(&history)?;
        let lowered_input = input.to_lowercase();

        if lowered_input == "q" || lowered_input == "quit" || lowered_input == "exit" {
            return Ok(());
        }

        history.insert(&input);
    }
}

fn get_input(history: &Trie) -> Result<String> {
    print_prompt()?;

    let mut line_buffer = String::new();
    while let Event::Key(event) = read()? {
        let mut input = None;
        match event {
            KeyEvent {
                modifiers: KeyModifiers::CONTROL,
                code: KeyCode::Char('c'),
            } => {
                process::exit(0);
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
                    .queue(terminal::Clear(ClearType::UntilNewLine))?
                    .flush()?;
            }
            KeyEvent {
                code: KeyCode::Tab, ..
            } => {
                println!();
                for word in history.words_with_prefix(&line_buffer) {
                    stdout().queue(Print(format!("  {}", word).grey()))?;
                    println!();
                }
                print_prompt()?;
                print!("{}", line_buffer);
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

    Ok(line_buffer)
}

fn print_prompt() -> Result<()> {
    stdout().queue(Print("> ".yellow()))?.flush()?;

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
