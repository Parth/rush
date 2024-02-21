use crate::{errors::Res, rush::Rush};

impl Rush {
    pub fn execute(&mut self) -> Res<()> {
        match self.prompt.as_ref() {
            "cd .." => {
                self.pwd.pop();
                self.prompt = String::new();
                Ok(())
            }
            _ => Err("unsupported".into()),
        }
    }
}
