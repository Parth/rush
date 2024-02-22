use crate::{error::Res, rush::Rush};

impl Rush {
    pub fn execute(&mut self) -> Res<()> {
        match self.input.as_ref() {
            "cd .." => {
                self.pwd.pop();
                self.input = String::new();
                Ok(())
            }
            _ => Err("unsupported".into()),
        }
    }
}
