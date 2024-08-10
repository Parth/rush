use std::io::{stdout, Write};

use crate::{
    error::Res,
    history::HistoryDataActions,
    rush::{KeyMode, Rush},
};
use crossterm::{
    event::{
        DisableBracketedPaste, DisableFocusChange, DisableMouseCapture, EnableBracketedPaste,
        EnableFocusChange, EnableMouseCapture, Event, EventStream, KeyCode, KeyEvent, KeyEventKind,
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
        stdout().queue(DisableMouseCapture)?;
        stdout().queue(EnableBracketedPaste)?;

        stdout().flush()?;

        self.event_loop().await?;

        terminal::disable_raw_mode()?;

        stdout().queue(DisableFocusChange)?;
        // stdout().queue(DisableMouseCapture)?;
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
                        Event::Paste(s) => {
                            self.append_input(&s);
                        }
                        Event::Key(KeyEvent {
                            code,
                            modifiers: KeyModifiers::NONE | KeyModifiers::SHIFT,
                            kind: KeyEventKind::Press,
                            state: _,
                        }) => match code {
                            KeyCode::Esc => {
                                self.mode = KeyMode::Insert;
                            }
                            KeyCode::Left => {
                                self.cursor_move_left(false);
                            }
                            KeyCode::Right => {
                                self.cursor_move_right(false);
                            }
                            KeyCode::Up => self.hist_prev(),
                            KeyCode::Down => self.hist_next(),
                            KeyCode::Char(c) => match self.mode {
                                KeyMode::Insert => self.append_char(c),
                                KeyMode::Suggest => self.suggest_mode(c)?,
                            }
                            KeyCode::Backspace => self.backspace()?,
                            KeyCode::Enter => self.parse(true)?,
                            KeyCode::F(n) => self.fn_key(n)?,
                            _ => continue,
                        },
                        Event::Key(KeyEvent {
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
    }
}
