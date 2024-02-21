use std::io::{self, stdout, Write};

use crossterm::{
    cursor::MoveToColumn,
    event::{
        self, DisableBracketedPaste, DisableFocusChange, DisableMouseCapture, EnableBracketedPaste,
        EnableFocusChange, EnableMouseCapture, KeyCode, KeyEvent, KeyEventKind,
    },
    terminal::{self},
    QueueableCommand,
};

struct Rush {
    prompt: String,
}

impl Rush {
    fn show(&self) -> io::Result<()> {
        stdout().queue(MoveToColumn(0))?;
        write!(stdout(), "$ {}", self.prompt)?;
        stdout().flush()?;
        Ok(())
    }

    fn execute(&self) -> io::Result<()> {
        Err(io::Error::new(
            io::ErrorKind::Unsupported,
            "support executing commands",
        ))
    }
}

fn main() -> io::Result<()> {
    terminal::enable_raw_mode()?;

    stdout().queue(EnableFocusChange)?;
    stdout().queue(EnableMouseCapture)?;
    stdout().queue(EnableBracketedPaste)?;

    stdout().flush()?;

    let mut rush = Rush {
        prompt: String::default(),
    };

    rush.show()?;

    loop {
        match event::read()? {
            event::Event::Key(KeyEvent {
                code,
                modifiers: _,
                kind: KeyEventKind::Press,
                state: _,
            }) => match code {
                KeyCode::Char(c) => {
                    rush.prompt = format!("{}{c}", rush.prompt);
                }
                KeyCode::Enter => rush.execute()?,
                _ => break,
            },
            event::Event::Mouse(_) => break,
            _ => continue,
        }

        rush.show()?;
    }

    terminal::disable_raw_mode()?;

    stdout().queue(DisableFocusChange)?;
    stdout().queue(DisableMouseCapture)?;
    stdout().queue(DisableBracketedPaste)?;

    stdout().flush()?;
    Ok(())
}
