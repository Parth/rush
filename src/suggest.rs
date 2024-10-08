use std::io::{stdout, Write};

use crossterm::{
    cursor::{position, MoveDown, MoveTo, MoveToNextLine, MoveUp, RestorePosition, SavePosition},
    terminal::{window_size, Clear, ClearType, ScrollUp},
    QueueableCommand,
};

use crate::{
    error::Res,
    rush::{KeyMode, Rush},
};

#[derive(Default)]
pub struct Suggest {
    pub suggestions: Vec<String>,
}

impl Rush {
    fn generate(&mut self) {
        self.suggest.suggestions.clear();
        let entries = vec![];
        let entries = self.history.entries.get(&self.pwd).unwrap_or(&entries);
        let history = entries.iter().rev().take(10);
        for hist in history {
            // one day, when we have more samples, this will be replaced with the fuzzy algorithm
            if hist.cmd.starts_with(&self.parser.input) {
                self.suggest.suggestions.push(hist.cmd.clone());
            }
        }
    }

    pub fn show_suggestions(&mut self) -> Res<()> {
        self.generate();
        stdout().flush()?;
        let height = window_size()?.rows;
        let room_reqd = 10;
        let room_left = height - position()?.1;
        let adjust = room_reqd - room_left;

        if adjust > 0 {
            stdout().queue(ScrollUp(adjust))?;
            stdout().queue(MoveUp(adjust))?;
        }

        let initial_pos = position()?;
        stdout().queue(MoveTo(0, initial_pos.1 + 1))?;
        stdout().queue(Clear(ClearType::FromCursorDown))?;

        for sug in &self.suggest.suggestions {
            stdout().queue(Clear(ClearType::CurrentLine))?;
            write!(stdout(), "{}", sug)?;
            stdout().queue(MoveToNextLine(1))?;
        }

        stdout().queue(MoveTo(initial_pos.0, initial_pos.1))?;
        stdout().flush()?;

        Ok(())
    }

    pub fn suggest_mode(&mut self, c: char) -> Res<()> {
        match c {
            '0' => self.accept_suggestion(10),
            '1'..='9' => self.accept_suggestion(c.to_digit(10).unwrap() as u8),
            _ => Ok(()),
        }
    }

    pub fn accept_suggestion(&mut self, n: u8) -> Res<()> {
        self.parser.input = self.suggest.suggestions[(n - 1) as usize].clone();
        self.show_prompt()?;
        self.parse(true)?;
        self.cursor.clear();
        self.mode = KeyMode::Insert;
        Ok(())
    }
}
