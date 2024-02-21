use std::{
    env,
    io::{stdout, Write},
    path::PathBuf,
};

use crossterm::{
    event::{
        self, DisableBracketedPaste, DisableFocusChange, DisableMouseCapture, EnableBracketedPaste,
        EnableFocusChange, EnableMouseCapture, KeyCode, KeyEvent, KeyEventKind,
    },
    terminal, QueueableCommand,
};

use crate::errors::Res;

pub struct Rush {
    pub pwd: PathBuf,
    pub home: PathBuf,
    pub prompt: String,
}

impl Rush {
    pub fn new() -> Rush {
        let home = env::var("HOME")
            .or(env::var("HOMEPATH"))
            .map(PathBuf::from)
            .expect("failed to detect home directory"); // todo: handle error

        let pwd = env::current_dir().unwrap_or_else(|_| home.clone());
        Self {
            pwd,
            home,
            prompt: String::new(),
        }
    }

    pub fn start_event_loop(&mut self) -> Res<()> {
        terminal::enable_raw_mode()?;

        stdout().queue(EnableFocusChange)?;
        stdout().queue(EnableMouseCapture)?;
        stdout().queue(EnableBracketedPaste)?;

        stdout().flush()?;

        self.event_loop()?;

        terminal::disable_raw_mode()?;

        stdout().queue(DisableFocusChange)?;
        stdout().queue(DisableMouseCapture)?;
        stdout().queue(DisableBracketedPaste)?;

        stdout().flush()?;

        Ok(())
    }

    fn event_loop(&mut self) -> Res<()> {
        self.show()?;
        loop {
            match event::read()? {
                event::Event::Key(KeyEvent {
                    code,
                    modifiers: _,
                    kind: KeyEventKind::Press,
                    state: _,
                }) => match code {
                    KeyCode::Char(c) => {
                        self.prompt = format!("{}{c}", self.prompt);
                    }
                    KeyCode::Enter => self.execute()?,
                    _ => break,
                },
                event::Event::Mouse(_) => break,
                _ => continue,
            }

            self.show()?;
        }

        Ok(())
    }
}
