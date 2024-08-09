use std::time::SystemTime;
use std::vec;
use std::{collections::HashMap, path::PathBuf, time::Instant};

use crate::{error::Res, rush::Rush};
use base64::{engine::general_purpose::URL_SAFE, Engine as _};
use serde::{Deserialize, Serialize};
use tokio::fs::{self, File, OpenOptions};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::mpsc::{self, UnboundedReceiver};

#[derive(Default)]
pub struct History {
    pub entries: HashMap<PathBuf, Vec<HistoryEntry>>,
    pub idx: Option<usize>,
}

#[derive(PartialEq, Eq, Serialize, Deserialize, Clone)]
pub struct HistoryEntry {
    pub cmd: String,
    pub last_run: u64,
    pub pass_count: u32,
    pub fail_count: u32,
}

pub enum HistoryDataActions {
    UpdateEntries(PathBuf, Vec<HistoryEntry>),
}

impl Rush {
    pub async fn start_disk_worker(&mut self) -> UnboundedReceiver<HistoryDataActions> {
        let (tx, rx) = mpsc::unbounded_channel();
        tokio::spawn(async move {
            loop {
                Self::refesh_history(tx.clone()).await;
            }
        });

        rx
    }

    pub async fn refesh_history(tx: mpsc::UnboundedSender<HistoryDataActions>) {
        fs::create_dir_all("/Users/parth/.rush/history/")
            .await
            .unwrap();
        let mut entries = fs::read_dir("/Users/parth/.rush/history").await.unwrap();
        let mut path_updates: HashMap<PathBuf, SystemTime> = HashMap::new();

        while let Some(child) = entries.next_entry().await.unwrap() {
            let meta = child.metadata().await.unwrap();

            let last_updated = meta.modified().unwrap();
            if let Some(entries) = path_updates.get_mut(&child.path()) {
                if *entries < last_updated {
                    tokio::spawn(Self::read_history_file(child.path(), tx.clone()));
                }
            } else {
                tokio::spawn(Self::read_history_file(child.path(), tx.clone()));
            }

            path_updates.insert(child.path(), last_updated);
        }
    }

    async fn read_history_file(path: PathBuf, tx: mpsc::UnboundedSender<HistoryDataActions>) {
        let mut f = File::open(&path).await.unwrap();
        let name = path.file_name().unwrap();
        let name = name.to_str().unwrap();
        let name = URL_SAFE.decode(name.as_bytes()).unwrap();
        let name = String::from_utf8(name).unwrap();
        let name = PathBuf::from(name);

        let mut content = vec![];
        f.read_to_end(&mut content).await.unwrap();
        let entries: Vec<HistoryEntry> = bincode::deserialize(&content).unwrap();
        tx.send(HistoryDataActions::UpdateEntries(name, entries))
            .unwrap();
    }

    // todo: handle ordered insertion
    // todo: deal with paths
    pub fn hist_add_input(&mut self) {
        self.history.idx = None;

        let cmd = self.parser.input.clone();
        let pwd = self.pwd.clone();

        let mut entry = self.history.entries.remove(&pwd).unwrap_or_default();
        let key = URL_SAFE.encode(pwd.to_str().unwrap().as_bytes());
        let mut path = PathBuf::from("/Users/parth/.rush/history");
        let mut added = false;
        for e in entry.iter_mut() {
            if e.cmd == cmd {
                e.last_run = Instant::now().elapsed().as_millis() as u64;
                e.pass_count += 1;
                added = true;
                break;
            }
        }

        if !added {
            entry.push(HistoryEntry {
                cmd,
                last_run: Instant::now().elapsed().as_millis() as u64,
                pass_count: 1,
                fail_count: 0,
            });
        }

        let content = bincode::serialize(&entry).unwrap();

        self.history.entries.insert(pwd, entry);

        path.push(key);
        tokio::spawn(async move {
            // todo: handle atomic writes
            let mut f = OpenOptions::new()
                .write(true)
                .create(true)
                .open(path)
                .await
                .unwrap();

            f.write_all(&content).await.unwrap();
        });
    }

    pub fn hist_prev(&mut self) {
        let entries = vec![];
        let entries = self.history.entries.get(&self.pwd).unwrap_or(&entries);

        if entries.is_empty() {
            return;
        }

        match self.history.idx {
            Some(mut idx) => {
                self.history.idx = {
                    idx = idx.saturating_sub(1);
                    Some(idx)
                }
            }
            None => self.history.idx = Some(entries.len() - 1),
        };

        self.parser.input = entries[self.history.idx.unwrap()].cmd.clone();
        self.cursor.clear();
    }

    pub fn hist_next(&mut self) {
        let entries = vec![];
        let entries = self.history.entries.get(&self.pwd).unwrap_or(&entries);

        if entries.is_empty() {
            return;
        }

        match self.history.idx {
            Some(idx) => {
                if idx == entries.len() - 1 {
                    self.history.idx = None;
                } else {
                    self.history.idx = Some(idx + 1);
                }
            }
            None => return,
        }

        match self.history.idx {
            Some(idx) => {
                self.parser.input = entries[idx].cmd.clone();
            }
            None => self.parser.input = String::new(),
        }

        self.cursor.clear();
    }

    pub fn do_hist(&mut self, n: u8) -> Res<()> {
        self.parser.input = self.suggest.suggestions[n as usize].clone();
        self.parse(true)?;
        self.cursor.clear();
        Ok(())
    }
}
