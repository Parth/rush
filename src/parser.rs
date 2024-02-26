use std::path::{Path, PathBuf};

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
    pub fn parse(&mut self, execute: bool) -> Res<()> {
        self.execute = execute;

        let mut tokens = self.input.split(' ');

        if let Some(command) = tokens.next() {
            match command {
                "cd" => Self::cd(tokens, &mut self.pwd, &self.home)?,
                _ => {}
            }
        }

        self.input.clear();
        self.cursor.clear();

        Ok(())
    }

    fn cd<'a, T>(tokens: T, pwd: &mut PathBuf, home: &Path) -> Res<()>
    where
        T: IntoIterator<Item = &'a str>,
    {
        match tokens.into_iter().next() {
            None => *pwd = home.to_path_buf(),
            Some(other) => {
                pwd.push(other);
                *pwd = pwd.canonicalize()?;
                // need to think through error propogation here
            }
        }

        Ok(())
    }
}
