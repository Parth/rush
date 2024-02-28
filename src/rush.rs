use std::{env, path::PathBuf};

use crate::cursor::Cursor;

pub struct Rush {
    pub cursor: Cursor,
    pub pwd: PathBuf,
    pub home: PathBuf,
    pub input: String,
    pub exit: bool,
}

impl Rush {
    pub fn new() -> Rush {
        let home = env::var("HOME")
            .or(env::var("HOMEPATH"))
            .map(PathBuf::from)
            .expect("failed to detect home directory"); // todo: handle error

        let pwd = env::current_dir().unwrap_or_else(|_| home.clone());

        Self {
            cursor: Default::default(),
            pwd,
            home,
            input: String::new(),
            exit: false,
        }
    }
}
