use std::io::{stdout, Write};

use crossterm::{
    cursor::{position, MoveToNextLine},
    event::{
        self, DisableBracketedPaste, DisableFocusChange, DisableMouseCapture, EnableBracketedPaste,
        EnableFocusChange, EnableMouseCapture, KeyCode, KeyEvent, KeyEventKind,
    },
    terminal::{self, window_size, ScrollUp},
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
        self.show()?;
        loop {
            match event::read()? {
                event::Event::Key(KeyEvent {
                    code,
                    modifiers: _,
                    kind: KeyEventKind::Press,
                    state: _,
                }) => match code {
                    KeyCode::Left => {
                        self.cursor_move_left(false);
                    }
                    KeyCode::Right => {
                        self.cursor_move_right(false);
                    }
                    KeyCode::Char(c) => self.append_input(c)?,
                    KeyCode::Backspace => self.backspace()?,
                    KeyCode::Enter => self.parse(true)?,
                    _ => break,
                },
                _ => break,
            }

            if self.execute {
                stdout().queue(MoveToNextLine(1))?;
                if position()?.1 == window_size()?.rows - 1 {
                    stdout().queue(ScrollUp(1))?;
                }
                self.execute = false;
            }

            self.show()?;
            self.cursor_show()?;
            stdout().flush()?;
        }

        Ok(())
    }
}
