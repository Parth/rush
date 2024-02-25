use std::path::PathBuf;

use crate::{error::Res, rush::Rush};

impl Rush {
    // todo: shell built ins
    // todo: single commands
    // todo: env vars
    // todo: *
    // todo: &&'s
    // todo: Pipe redirects
    // todo: ( )
    // execute is a flag because the parser will also syntax highlight
    pub fn parse(&mut self, _execute: bool) -> Res<()> {
        let mut tokens = self.input.split(' ');

        if let Some(command) = tokens.next() {
            match command {
                "cd" => Self::cd(tokens, &mut self.pwd, &self.home)?,
                _ => {}
            }
        }

        self.input.clear();
        Ok(())
    }

    fn cd<'a, T>(tokens: T, pwd: &mut PathBuf, home: &PathBuf) -> Res<()>
    where
        T: IntoIterator<Item = &'a str>,
    {
        match tokens.into_iter().next() {
            None => *pwd = home.clone(),
            Some(other) => {
                pwd.push(other);
                *pwd = pwd.canonicalize()?;
                // need to think through error propogation here
            }
        }

        Ok(())
    }
}
