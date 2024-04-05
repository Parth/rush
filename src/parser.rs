use std::{
    ops::Range,
    process::{self},
};

use crossterm::terminal::{self};

use crate::{error::Res, rush::Rush};

pub enum ExpressionType {
    Builtin,
    //Command,
}

pub enum TokenType {
    And,
    Or,
    Expression(ExpressionType),
}

// the plan here is to parse all the information into this format, unfortunately the ranges here
// are going to be word indexes into the shellsplit words instead of character indexes into the
// underlying string, this is going to make syntax highlighting more annoying. The path forward
// here will be to re-write that part of the parser and also return indexes, or incorporate more
// sophisticated token ids into our parse tree.
pub struct Token {
    pub t_type: TokenType,
    pub range: Range<usize>,
}

// this could be another approach: you have a command that consumes itself, all the args and any
// operators between itself and the next command, based on it's status code and the next command
// type, it can evaluate whether continuation should happen. Potentially this is also the best way
// to describe pipes
pub enum Criteria {
    And,
    Or,
    Semilcolon,
}

pub struct Command {
    pub location: Range<usize>,
    pub status: u8, // todo consider enum
    pub cont: Option<Criteria>,
}

// both these approaches extend to a world with our own parser, in their own ways. And both
// approaches will require more help for syntax highlighting reliably. Although I wonder if these
// counts can be derived with some assumptions. It's probably the various forms of whitespace that
// pose the biggest problem. I'd also like shift enter to seamlessly enter multi line mode.
// Possibly some other enter combination can just re-run the last command.
//
// These approaches are also fundementally structuring the parsed tokens linearly, instead of in a
// tree, the only example of a situation I can think of where this matters is in Parentasis.

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
