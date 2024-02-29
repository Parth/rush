use std::{
    ffi::OsStr,
    path::{Path, PathBuf},
    process::{self, Command},
};

use crossterm::terminal::{self};

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
        if execute {
            Self::next_line()?;
        }

        // todo: we used iters prior to shell_words but now this is probably costing us flexibility
        // for no performance increase
        let tokens = shell_words::split(&self.input)?;
        let mut tokens = tokens.iter();

        if let Some(command) = tokens.next() {
            match command.as_ref() {
                "" => {}
                "cd" => Self::cd(tokens, &mut self.pwd, &self.home)?,
                "exit" => Self::exit(),
                c => {
                    if execute {
                        Self::command(c, tokens, &self.pwd)?
                    }
                }
            }
        }

        if execute {
            self.reset_prompt();
        }

        Ok(())
    }

    pub fn exit() -> ! {
        process::exit(0);
    }

    fn cd<P, T>(tokens: T, pwd: &mut PathBuf, home: &Path) -> Res<()>
    where
        T: IntoIterator<Item = P>,
        P: AsRef<Path>,
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

    fn command<P, T>(c: &str, tokens: T, pwd: &Path) -> Res<()>
    where
        T: IntoIterator<Item = P>,
        P: AsRef<OsStr>,
    {
        terminal::disable_raw_mode()?;

        let mut c = Command::new(c);
        c.current_dir(pwd);
        c.args(tokens);
        c.status()?;

        terminal::enable_raw_mode()?;

        Ok(())
    }
}
