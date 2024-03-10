use std::io::stdout;

use crossterm::{cursor, QueueableCommand};

use crate::{error::Res, rush::Rush};

#[derive(Default)]
pub struct Cursor {
    /// represents the first valid location after the prompt
    pub min_cursor: Option<u16>,

    pub cursor_location: Option<u16>,
    pub max_cursor: Option<u16>,
}

impl Rush {
    pub fn cursor_move_left(&mut self, contract_bounds: bool) {
        let min = self.cursor.min_cursor.as_mut().unwrap();
        if contract_bounds {
            *self.cursor.max_cursor.as_mut().unwrap() -= 1;
        }

        let loc = self.cursor.cursor_location.as_mut().unwrap();
        if loc > min {
            *loc -= 1;
        }
    }

    // when we want to support emojis we'll need to do the following:
    // we need to calculate what the next column we're about to move to is
    // despite a monospaced environment there are characters that are multiple columns wide 🎉
    // we can use UnicodeWidth to determine which characters behave this way.
    // we can use grapheme indexes to iterate through the string. I thought about doing this now
    // but for whom is emoji a higher priority than history and other such things? I'd like to
    // meet such a person.
    pub fn cursor_move_right(&mut self, expand: bool) {
        let max = self.cursor.max_cursor.as_mut().unwrap();
        let loc = self.cursor.cursor_location.as_mut().unwrap();

        if expand {
            *max += 1;
        }

        if loc < max {
            *loc += 1;
        }
    }

    pub fn cursor_show(&self) -> Res<()> {
        stdout().queue(cursor::MoveToColumn(self.cursor.cursor_location.unwrap()))?;
        Ok(())
    }
}

impl Cursor {
    pub fn clear(&mut self) {
        self.min_cursor = None;
        self.cursor_location = None;
        self.max_cursor = None;
    }
}
