use std::io::{stdout, Write};

use crossterm::{
    cursor::{position, MoveDown, MoveTo, MoveToNextLine, MoveUp, RestorePosition, SavePosition},
    terminal::{window_size, Clear, ClearType, ScrollUp},
    QueueableCommand,
};

use crate::{error::Res, rush::Rush};

#[derive(Default)]
pub struct Suggest {
    suggestions: Vec<String>,
}

impl Rush {
    fn generate(&mut self) {
        self.suggest.suggestions.clear();
        let history = self.history.entries.iter().rev().take(10);
        for hist in history {
            self.suggest.suggestions.push(hist.clone());
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

        for sug in &self.suggest.suggestions {
            stdout().queue(Clear(ClearType::CurrentLine))?;
            write!(stdout(), "{}", sug)?;
            stdout().queue(MoveToNextLine(1))?;
        }

        stdout().queue(MoveTo(initial_pos.0, initial_pos.1))?;
        stdout().flush()?;

        Ok(())
    }
}
