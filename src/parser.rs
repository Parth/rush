use std::{mem, ops::Range, process};

use crate::{error::Res, rush::Rush};

#[derive(Default)]
pub struct Parser {
    pub execute: bool,
    pub input: String,
    pub output: ParserOutput,
}

/// A struct optimized for command execution planning and syntax highlighting
#[derive(Default)]
pub struct ParserOutput {
    commands: Vec<Cmd>,
    problems: Vec<ParseProblem>,
}

/// Represents everything the parser understood
#[derive(Default)]
pub struct Cmd {
    pub total_range: Range<usize>,

    pub var: Vec<Var>,

    pub name: String,
    pub name_range: Range<usize>,

    pub args: Vec<Arg>,

    pub consumed: Vec<ConsumedChars>,

    pub next: Option<Next>,

    pub stage: CmdStage,
}

pub struct Var {}

pub struct Arg {
    pub val: String,
    pub range: Range<usize>,
}

#[derive(Default)]
pub enum CmdStage {
    #[default]
    NotRun,
    Running,
    Finished(u8),
}

pub struct Next {
    next_type: NextType,
    range: Range<usize>,
}

pub enum NextType {
    And,
    Or,
    Semi,
}

pub struct ConsumedChars {
    c: char,
    idx: usize,
}

/// Represents the current behavior of the parser
/// Takes inspiration from shell_words
#[derive(Debug)]
pub enum ParserState {
    SingleQuote(usize),
    DoubleQuote(usize),
    Delimeter,
    Command,
    Args,
}

pub enum ParseProblem {
    UnmatchedQuote(usize),
}

impl Rush {
    pub fn parse(&mut self, execute: bool) -> Res<()> {
        self.parser.execute = execute;
        if self.parser.execute {
            Self::next_line()?;
        }

        let out = self.parse_loop();
        // time to execute

        if self.parser.execute {
            self.hist_add_input();
            self.reset_prompt();
        }

        Ok(())
    }

    fn parse_loop(&self) -> ParserOutput {
        let mut state = vec![];
        let mut commands = vec![];
        let mut command = Cmd::default();

        let chars = self.parser.input.chars().enumerate();
        for (idx, c) in chars {
            match state.as_slice() {
                [] => match c {
                    '\'' => {
                        state.push(ParserState::SingleQuote(idx));
                        command.consumed.push(ConsumedChars { c, idx })
                    }
                    '"' => {
                        state.push(ParserState::DoubleQuote(idx));
                        command.consumed.push(ConsumedChars { c, idx });
                    }
                    _ => {
                        state.push(ParserState::Command);
                        command.name.push(c)
                    }
                },
                [ParserState::Command] => match c {
                    ' ' => {
                        // maybe this should be a push
                        state.push(ParserState::Delimeter);
                    }
                    '=' => todo!("change from name to env vars here"),
                    _ => {
                        command.name.push(c);
                    }
                },
                [ParserState::Command, ParserState::Delimeter] => match c {
                    '"' => {
                        state[1] = ParserState::Args;
                        command.args.push(Arg {
                            val: String::from(c),
                            range: idx..idx + 1,
                        });
                        state.push(ParserState::DoubleQuote(idx));
                    }
                    _ => {
                        state[1] = ParserState::Args;
                        command.args.push(Arg {
                            val: String::from(c),
                            range: idx..idx + 1,
                        })
                    }
                },
                [ParserState::Command, ParserState::Args] => match c {
                    ' ' => {
                        state[1] = ParserState::Delimeter;
                    }
                    _ => {
                        command.args.last_mut().unwrap().val.push(c);
                        command.args.last_mut().unwrap().range.end += 1;
                    }
                },

                _ => panic!("unhandled parser state\n{:#?}", state),
            }
        }

        let mut problems = vec![];
        let mut command_count = 0;
        for s in state {
            match s {
                ParserState::SingleQuote(loc) => problems.push(ParseProblem::UnmatchedQuote(loc)),
                ParserState::DoubleQuote(loc) => problems.push(ParseProblem::UnmatchedQuote(loc)),
                ParserState::Command => {
                    if command_count == 0 {
                        commands.push(mem::replace(&mut command, Cmd::default()));
                    } else {
                        panic!("multiple outstanding commands found");
                    }
                    command_count += 1;
                }
                ParserState::Delimeter => {}
                ParserState::Args => todo!(),
            }
        }

        ParserOutput { commands, problems }
    }

    pub fn exit() -> ! {
        process::exit(0);
    }
}
