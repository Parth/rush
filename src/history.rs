use crate::rush::Rush;

#[derive(Default)]
pub struct History {
    entries: Vec<String>,
    history_idx: Option<usize>,
}

impl Rush {
    pub fn push_entry(&mut self) {}

    pub fn prev(&mut self) -> Option<&str> {
        todo!()
    }
}
