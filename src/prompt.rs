use std::{
    io::{stdout, Write},
    path::PathBuf,
};

use crossterm::{
    cursor::{self, position, MoveToColumn, MoveToNextLine},
    style::{self, Attribute, Attributes, ContentStyle, ResetColor, SetForegroundColor, SetStyle},
    terminal::{self, window_size, Clear, ScrollUp},
    QueueableCommand,
};

use crate::{error::Res, rush::Rush};

impl Rush {
    pub fn next_line() -> Res<()> {
        stdout().queue(MoveToNextLine(1))?;
        if position()?.1 == window_size()?.rows - 1 {
            stdout().queue(ScrollUp(1))?;
        }
        stdout().flush()?;
        Ok(())
    }

    pub fn reset_prompt(&mut self) {
        self.parser.input.clear();
        self.cursor.clear();
    }

    pub fn shortened_home(&self) -> Option<PathBuf> {
        let mut trim_home = true;
        let mut pwd = self.pwd.iter();
        for h in self.home.iter() {
            let mut matched = false;
            if let Some(p) = pwd.next() {
                matched = h == p
            }

            if !matched {
                trim_home = false;
                break;
            }
        }

        if !trim_home {
            return None;
        }

        let mut new_pwd = PathBuf::from("~");
        for path in pwd {
            new_pwd.push(path);
        }

        Some(new_pwd)
    }

    fn show_pwd(&self) -> Res<()> {
        stdout().queue(SetForegroundColor(style::Color::DarkBlue))?;
        let shortened = self.shortened_home();
        let pwd = shortened.as_ref().unwrap_or(&self.pwd).to_str().unwrap();
        write!(stdout(), "{}", pwd)?;
        stdout().queue(ResetColor)?;
        Ok(())
    }

    fn show_input(&mut self) -> Res<()> {
        stdout().queue(SetStyle(ContentStyle {
            foreground_color: None,
            background_color: None,
            underline_color: None,
            attributes: Attributes::from(Attribute::Bold),
        }))?;
        write!(stdout(), " > ")?;
        stdout().queue(ResetColor)?;

        if self.cursor.min_cursor.is_none() {
            stdout().flush()?;
            self.cursor.min_cursor = Some(cursor::position()?.0);
            self.cursor.max_cursor = self.cursor.min_cursor;
        }

        write!(stdout(), "{}", self.parser.input)?;

        if self.cursor.cursor_location.is_none() {
            stdout().flush()?;
            self.cursor.cursor_location = Some(cursor::position()?.0);
        }
        Ok(())
    }

    pub fn show(&mut self) -> Res<()> {
        stdout().queue(Clear(terminal::ClearType::CurrentLine))?;
        stdout().queue(MoveToColumn(0))?;
        self.show_pwd()?;
        self.show_input()?;
        Ok(())
    }

    pub fn append_char(&mut self, c: char) {
        let current_index = self.cursor.cursor_location.unwrap() - self.cursor.min_cursor.unwrap();
        let current_index = current_index as usize;

        self.parser.input.insert(current_index, c);
        self.cursor_move_right(true);
    }

    pub fn append_input(&mut self, s: &str) {
        let current_index = self.cursor.cursor_location.unwrap() - self.cursor.min_cursor.unwrap();
        let current_index = current_index as usize;

        self.parser.input.insert_str(current_index, &s);
        for _ in 0..s.len() {
            self.cursor_move_right(true);
        }
    }

    pub fn backspace(&mut self) -> Res<()> {
        if self.parser.input.is_empty() {
            return Ok(()); // todo: boop here
        }

        let current_index = self.cursor.cursor_location.unwrap() - self.cursor.min_cursor.unwrap();
        let current_index = current_index as usize;

        self.parser
            .input
            .replace_range(current_index - 1..current_index, "");
        self.cursor_move_left(true);
        Ok(())
    }
}
