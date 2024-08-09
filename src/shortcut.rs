use crossterm::event::{KeyCode, KeyModifiers};

use crate::{error::Res, rush::Rush};

impl Rush {
    pub fn shortcut(&mut self, code: KeyCode, modifier: KeyModifiers) -> Res<()> {
        match (modifier, code) {
            (KeyModifiers::CONTROL, KeyCode::Char('d')) => {
                Self::next_line()?;
                Self::exit();
            }
            _ => {}
        }

        Ok(())
    }

    pub fn fn_key(&mut self, n: u8) -> Res<()> {
        self.do_hist(n)
    }
}
