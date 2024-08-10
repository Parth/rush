use crossterm::event::{KeyCode, KeyModifiers};

use crate::{
    error::Res,
    rush::{KeyMode, Rush},
};

impl Rush {
    pub fn shortcut(&mut self, code: KeyCode, modifier: KeyModifiers) -> Res<()> {
        match (modifier, code) {
            (KeyModifiers::CONTROL, KeyCode::Char('s')) => {
                if self.mode == KeyMode::Insert {
                    self.mode = KeyMode::Suggest;
                }
            }
            (KeyModifiers::CONTROL, KeyCode::Char('d')) => {
                Self::next_line()?;
                Self::exit();
            }
            _ => {}
        }

        Ok(())
    }

    pub fn fn_key(&mut self, n: u8) -> Res<()> {
        self.accept_suggestion(n)
    }
}
