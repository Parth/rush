use rush::Rush;

mod cursor;
mod error;
mod event;
mod history;
mod parser;
mod prompt;
mod rush;
mod shortcut;
mod sugguest;

// Ideally, this detail will be hidden from users in the future
#[tokio::main]
async fn main() {
    Rush::new().start_event_loop().await.unwrap();
}

// todo: env vars
// todo: pipes
// todo: operators
