use std::{
    ffi::OsStr,
    path::{Path, PathBuf},
    process::{self, Command},
};

use crossterm::terminal::{self};

use crate::{error::Res, rush::Rush};

#[derive(Default)]
pub struct Parser {
    pub execute: bool,
    pub input: String,
    // todo: these next two fields will likely become some sort of execution plan
    pub word_idx: usize,
    pub shell_words: Vec<String>,
}

impl Rush {
    // todo: env vars
    // todo: *
    // todo: &&'s
    // todo: Pipe redirects
    // todo: ( )
    // execute is a flag because the parser will also syntax highlight
    pub fn parse(&mut self, execute: bool) -> Res<()> {
        self.parser.execute = execute;
        if self.parser.execute {
            Self::next_line()?;
        }

        // todo: we used iters prior to shell_words but now this is probably costing us flexibility
        // for no performance increase
        self.parser.shell_words = shell_words::split(&self.parser.input)?;

        self.parser.word_idx = 0;
        if let Some(command) = self.parser.shell_words.get(self.parser.word_idx) {
            match command.as_ref() {
                "" => {} // needed?
                "cd" => self.cd()?,
                "exit" => Self::exit(),
                _ => {
                    // these can be moved locally now
                    if self.parser.execute {
                        self.command()?;
                    }
                }
            }
        }

        if self.parser.execute {
            self.hist_add_input();
            self.reset_prompt();
        }

        Ok(())
    }

    pub fn exit() -> ! {
        process::exit(0);
    }

    fn cd(&mut self) -> Res<()> {
        self.parser.word_idx += 1;
        match self.parser.shell_words.get(self.parser.word_idx) {
            None => self.pwd = self.home.to_path_buf(),
            Some(other) => {
                self.pwd.push(other);
                self.pwd = self.pwd.canonicalize()?;
                // need to think through error propogation here
            }
        }

        Ok(())
    }

    fn command(&mut self) -> Res<()> {
        terminal::disable_raw_mode()?;

        let c = &self.parser.shell_words[self.parser.word_idx];
        self.parser.word_idx += 1;
        let mut c = Command::new(c);
        c.current_dir(&self.pwd);
        c.args(&self.parser.shell_words[self.parser.word_idx..]);
        c.status()?;

        terminal::enable_raw_mode()?;

        Ok(())
    }
}
