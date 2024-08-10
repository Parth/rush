use std::{env, path::PathBuf};

use crate::{config::Config, cursor::Cursor, history::History, parser::Parser, suggest::Suggest};

pub struct Rush {
    pub cfg: Config,
    pub mode: KeyMode,
    pub cursor: Cursor,
    pub pwd: PathBuf,
    pub home: PathBuf,
    pub parser: Parser,
    pub history: History,
    pub suggest: Suggest,
}

#[derive(Default, PartialEq, Eq)]
pub enum KeyMode {
    #[default]
    Insert,
    Suggest,
}

impl Rush {
    pub fn new(cfg: Config) -> Rush {
        let home = env::var("HOME")
            .or(env::var("HOMEPATH"))
            .map(PathBuf::from)
            .expect("failed to detect home directory"); // todo: handle error

        let pwd = env::current_dir().unwrap_or_else(|_| home.clone());

        Self {
            cfg,
            cursor: Cursor::default(),
            pwd,
            home,
            history: History::default(),
            parser: Parser::default(),
            suggest: Suggest::default(),
            mode: KeyMode::default(),
        }
    }
}
