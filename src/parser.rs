use std::{
    io::ErrorKind,
    ops::Range,
    process::{self, Command},
};

use crossterm::terminal;

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
    /// the input this parsing output is associated with, allows fast determination if
    /// re-parsing is needed
    input: String,
    commands: Vec<Cmd>,
    problems: Vec<ParseProblem>,
}

/// Represents everything the parser understood
#[derive(Default, Clone)]
pub struct Cmd {
    pub total_range: Range<usize>,

    pub var: Vec<Var>,

    pub name: String,
    pub name_range: Range<usize>,

    pub args: Vec<Arg>,

    pub consumed: Vec<ConsumedChars>,

    pub next: Option<Next>,

    pub stage: CmdStage,
    pub background: bool,
}

#[derive(Clone)]
pub struct Var {}

#[derive(Clone)]
pub struct Arg {
    pub val: String,
    pub range: Range<usize>,
}

#[derive(Default, Clone)]
pub enum CmdStage {
    #[default]
    NotRun,
    Running,
    Finished(u8),
}

#[derive(Clone)]
pub struct Next {
    next_type: NextType,
    range: Range<usize>,
}

#[derive(Clone)]
pub enum NextType {
    And,
    Or,
    Semi,
}

#[derive(Clone)]
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

    /// how many & characters have we seen (bg or &&)?
    And {
        count: usize,
    },
}

pub enum ParseProblem {
    UnmatchedQuote(usize),
    TrailingAnd,
}

impl Rush {
    pub fn parse(&mut self, execute: bool) -> Res<()> {
        self.parser.execute = execute;
        if self.parser.execute {
            Self::next_line()?;
        }

        if self.parser.input != self.parser.output.input {
            self.parser.output = self.parse_fsm();
        }

        if self.parser.execute {
            for cmd in self.parser.output.commands.clone() {
                terminal::disable_raw_mode()?;
                let onwards = self.command(cmd)?;
                terminal::enable_raw_mode()?;

                if !onwards {
                    break;
                }
            }

            self.hist_add_input();
            self.reset_prompt();
        }

        Ok(())
    }

    fn parse_fsm(&self) -> ParserOutput {
        let input = self.parser.input.clone();
        let mut state = vec![];
        let mut commands = vec![Cmd::default()];

        let chars = input.chars().enumerate();
        for (idx, c) in chars {
            let last = commands.len() - 1;
            match state.as_slice() {
                [] => match c {
                    ' ' => {
                        continue;
                    }
                    _ => {
                        state.push(ParserState::Command);
                        commands[last].name.push(c)
                    }
                },
                [ParserState::Command] => match c {
                    ' ' => {
                        // todo: what if no name yet
                        state.push(ParserState::Delimeter);
                    }
                    '=' => todo!("change from name to env vars here"),
                    _ => {
                        // possibly this state should have it's own identifier, if we upgrade to an
                        // env var or something of that nature, the Command, Delimiter branch will
                        // assume data is going to args.
                        commands[last].name.push(c);
                    }
                },
                [ParserState::Command, ParserState::Delimeter] => match c {
                    '"' => {
                        state[1] = ParserState::Args;
                        state.push(ParserState::DoubleQuote(idx));
                        commands[last].args.push(Arg {
                            val: String::default(),
                            range: idx..idx + 1,
                        });
                    }
                    '&' => {
                        state[1] = ParserState::And { count: 1 };
                    }
                    _ => {
                        state[1] = ParserState::Args;
                        commands[last].args.push(Arg {
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
                        let arg = commands[last].args.last_mut().unwrap();
                        arg.val.push(c);
                        arg.range.end += 1;
                    }
                },
                [ParserState::Command, ParserState::And { count: 1 }] => match c {
                    '&' => {
                        commands.push(Cmd::default());
                        state.clear();
                    }
                    _ => panic!("unreasonable character found after & {c}"),
                },
                [ParserState::Command, ParserState::Args, ParserState::DoubleQuote(_)] => match c {
                    '"' => {
                        state.pop();
                    }
                    _ => {
                        let arg = commands[last].args.last_mut().unwrap();
                        arg.val.push(c);
                        arg.range.end += 1;
                    }
                },
                _ => panic!("unhandled parser state\n{:#?}", state),
            }
        }

        let mut problems = vec![];
        for s in state {
            match s {
                ParserState::SingleQuote(loc) => problems.push(ParseProblem::UnmatchedQuote(loc)),
                ParserState::DoubleQuote(loc) => problems.push(ParseProblem::UnmatchedQuote(loc)),
                ParserState::Command => {}
                ParserState::Args => {}
                ParserState::Delimeter => {}
                ParserState::And { count: 1 } => {
                    commands.last_mut().unwrap().background = true;
                }
                ParserState::And { count: _ } => {
                    problems.push(ParseProblem::TrailingAnd);
                }
            }
        }

        ParserOutput {
            input,
            commands,
            problems,
        }
    }

    fn command(&mut self, cmd: Cmd) -> Res<bool> {
        let status = match cmd.name.as_str() {
            "cd" => self.cd(cmd)?,
            _ => {
                let mut c = Command::new(&cmd.name);
                c.current_dir(&self.pwd);
                c.args(cmd.args.iter().map(|arg| &arg.val));
                match c.status() {
                    Ok(status) => status.code().unwrap_or(-1),
                    Err(failed) => match failed.kind() {
                        // todo: would be nice for these to have colors
                        ErrorKind::NotFound => {
                            eprintln!("rush could not find the command: {:?}", &cmd.name);
                            127
                        }
                        _ => {
                            eprintln!("rush failed to run command: {}", failed);
                            -2
                        }
                    },
                }
            }
        };

        // todo: status will need to be stored somewhere ultimately
        Ok(status == 0)
    }

    fn cd(&mut self, cmd: Cmd) -> Res<i32> {
        let status = match cmd.args.as_slice() {
            [] => {
                self.pwd = self.home.clone();
                0
            }
            [dest] => {
                self.pwd.push(dest.val.clone());
                self.pwd = self.pwd.canonicalize()?;
                0
            }
            other => {
                println!(
                    "cd expected 1 argument but found more than one: {:?}",
                    other
                        .iter()
                        .map(|arg| &arg.val)
                        .collect::<Vec<&String>>()
                );
                1
            }
        };

        Ok(status)
    }

    pub fn exit() -> ! {
        process::exit(0);
    }
}

// error situations:
//
// - invalid input during non execution parsing -- should be ignored, don't stop parsing
// - invalid input during execution -- should stop parsing and
