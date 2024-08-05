use std::io::{stdout, Write};

use crossterm::{
    event::{
        self, DisableBracketedPaste, DisableFocusChange, DisableMouseCapture, EnableBracketedPaste,
        EnableFocusChange, EnableMouseCapture, KeyCode, KeyEvent, KeyEventKind, KeyModifiers,
    },
    terminal::{self},
    QueueableCommand,
};

use crate::{error::Res, rush::Rush};

impl Rush {
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
        self.show_prompt()?;
        loop {
            match event::read()? {
                event::Event::Paste(s) => {
                    self.append_input(&s);
                }
                event::Event::Key(KeyEvent {
                    code,
                    modifiers: KeyModifiers::NONE | KeyModifiers::SHIFT,
                    kind: KeyEventKind::Press,
                    state: _,
                }) => match code {
                    KeyCode::Left => {
                        self.cursor_move_left(false);
                    }
                    KeyCode::Right => {
                        self.cursor_move_right(false);
                    }
                    KeyCode::Up => self.hist_prev(),
                    KeyCode::Down => self.hist_next(),
                    KeyCode::Char(c) => self.append_char(c),
                    KeyCode::Backspace => self.backspace()?,
                    KeyCode::Enter => self.parse(true)?,
                    _ => continue,
                },
                event::Event::Key(KeyEvent {
                    code,
                    modifiers,
                    kind: KeyEventKind::Press,
                    state: _,
                }) => self.shortcut(code, modifiers)?,
                _ => continue,
            }

            self.show_prompt()?;
            self.cursor_show()?;
            self.show_suggestions()?;
            stdout().flush()?;
        }
    }
}
