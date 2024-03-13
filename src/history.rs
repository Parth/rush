use crate::rush::Rush;

#[derive(Default)]
pub struct History {
    entries: Vec<String>,
    idx: Option<usize>,
}

impl Rush {
    pub fn hist_add_input(&mut self) {
        self.history.idx = None;
        self.history.entries.push(self.parser.input.clone());
    }

    pub fn hist_prev(&mut self) {
        if self.history.entries.is_empty() {
            return;
        }

        match self.history.idx {
            Some(mut idx) => {
                self.history.idx = {
                    if idx != 0 {
                        idx -= 1;
                    }
                    Some(idx)
                }
            }
            None => self.history.idx = Some(self.history.entries.len() - 1),
        };

        self.parser.input = self.history.entries[self.history.idx.unwrap()].clone();
        self.cursor.clear();
    }

    pub fn hist_next(&mut self) {
        if self.history.entries.is_empty() {
            return;
        }

        match self.history.idx {
            Some(idx) => {
                if idx == self.history.entries.len() - 1 {
                    self.history.idx = None;
                } else {
                    self.history.idx = Some(idx + 1);
                }
            }
            None => return,
        }

        match self.history.idx {
            Some(idx) => {
                self.parser.input = self.history.entries[idx].clone();
            }
            None => self.parser.input = String::new(),
        }

        self.cursor.clear();
    }
}
