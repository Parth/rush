use rush::Rush;

mod cursor;
mod error;
mod event;
mod history;
mod keymap;
mod parser;
mod prompt;
mod rush;

fn main() {
    Rush::new().start_event_loop().unwrap();
}

// todo: env vars
// todo: history
// todo: pipes
// todo: operators
