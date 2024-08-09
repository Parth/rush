use std::io::{stdout, Write};

use crate::{error::Res, history::HistoryDataActions, rush::Rush};
use crossterm::{
    event::{
        self, DisableBracketedPaste, DisableFocusChange, DisableMouseCapture, EnableBracketedPaste,
        EnableFocusChange, EnableMouseCapture, EventStream, KeyCode, KeyEvent, KeyEventKind,
        KeyModifiers,
    },
    terminal::{self},
    QueueableCommand,
};
use futures::{future::FutureExt, pin_mut, select, StreamExt};

impl Rush {
    pub async fn start_event_loop(&mut self) -> Res<()> {
        terminal::enable_raw_mode()?;

        stdout().queue(EnableFocusChange)?;
        stdout().queue(EnableMouseCapture)?;
        stdout().queue(EnableBracketedPaste)?;

        stdout().flush()?;

        self.event_loop().await?;

        terminal::disable_raw_mode()?;

        stdout().queue(DisableFocusChange)?;
        stdout().queue(DisableMouseCapture)?;
        stdout().queue(DisableBracketedPaste)?;

        stdout().flush()?;

        Ok(())
    }

    async fn event_loop(&mut self) -> Res<()> {
        self.show_prompt()?;

        let mut reader = EventStream::new();
        let mut data = self.start_disk_worker().await;

        loop {
            let mut event = reader.next().fuse();
            let data_rx = data.recv().fuse();
            pin_mut!(data_rx);

            select! {
                    maybe_data = data_rx => {
                        match maybe_data.unwrap() {
                            HistoryDataActions::UpdateEntries(path, entries) => {
                                self.history.entries.insert(path, entries);
                            }
                        }
                    }
                    maybe_event = event => {
                        match maybe_event.unwrap().unwrap() {
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
                                KeyCode::F(n) => self.fn_key(n)?,
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
            }}
        }
    }
}
