use std::{env, path::PathBuf, time::Duration};

pub struct Config {
    pub data_refresh_rest: Duration,
    pub data_dir: PathBuf,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            data_refresh_rest: Duration::from_millis(250),
            data_dir: Self::data_dir(),
        }
    }
}

impl Config {
    fn home() -> PathBuf {
        let home = env::var("HOME").or(env::var("HOMEPATH")).unwrap();
        PathBuf::from(home)
    }

    fn data_dir() -> PathBuf {
        let mut home = Self::home();
        home.push(".rush");
        home
    }

    pub fn hist_dir(&self) -> PathBuf {
        let mut data_dir = self.data_dir.clone();
        data_dir.push("history");
        data_dir
    }
}
