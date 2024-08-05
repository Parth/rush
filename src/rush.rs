use std::{env, path::PathBuf};

use crate::{cursor::Cursor, history::History, parser::Parser, sugguest::Suggest};

pub struct Rush {
    pub cursor: Cursor,
    pub pwd: PathBuf,
    pub home: PathBuf,
    pub parser: Parser,
    pub history: History,
    pub suggest: Suggest,
}

impl Rush {
    pub fn new() -> Rush {
        let home = env::var("HOME")
            .or(env::var("HOMEPATH"))
            .map(PathBuf::from)
            .expect("failed to detect home directory"); // todo: handle error

        let pwd = env::current_dir().unwrap_or_else(|_| home.clone());

        Self {
            cursor: Cursor::default(),
            pwd,
            home,
            history: History::default(),
            parser: Parser::default(),
            suggest: Suggest::default(),
        }
    }
}
