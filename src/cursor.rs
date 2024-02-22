use std::io::stdout;

use crossterm::{cursor, QueueableCommand};

use crate::{error::Res, rush::Rush};

#[derive(Default)]
pub struct Cursor {
    pub min_cursor: Option<u16>,
    pub cursor_location: Option<u16>,
    pub max_cursor: Option<u16>,
}

impl Rush {
    pub fn cursor_move_left(&mut self, contract_bounds: bool) {
        let min = self.cursor.min_cursor.as_mut().unwrap();
        if contract_bounds {
            *min -= 1;
        }

        let loc = self.cursor.cursor_location.as_mut().unwrap();
        if loc > min {
            *loc -= 1;
        }
    }

    pub fn cursor_move_right(&mut self, expand_bounds: bool) {
        let max = self.cursor.max_cursor.as_mut().unwrap();
        if expand_bounds {
            *max += 1;
        }

        let loc = self.cursor.cursor_location.as_mut().unwrap();
        if loc < max {
            *loc += 1;
        }
    }

    pub fn cursor_show(&self) -> Res<()> {
        stdout().queue(cursor::MoveToColumn(self.cursor.cursor_location.unwrap()))?;
        Ok(())
    }
}
