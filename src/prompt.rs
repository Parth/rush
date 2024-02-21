use std::{
    io::{stdout, Write},
    path::PathBuf,
};

use crossterm::{
    cursor::MoveToColumn,
    style::{self, Attribute, Attributes, ContentStyle, ResetColor, SetForegroundColor, SetStyle},
    terminal::{self, Clear},
    QueueableCommand,
};

use crate::{errors::Res, rush::Rush};

impl Rush {
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
        stdout().queue(SetForegroundColor(style::Color::Green))?;
        let shortened = self.shortened_home();
        let pwd = shortened.as_ref().unwrap_or(&self.pwd).to_str().unwrap();
        write!(stdout(), "{}", pwd)?;
        stdout().queue(ResetColor)?;
        Ok(())
    }

    fn show_input(&self) -> Res<()> {
        stdout().queue(SetStyle(ContentStyle {
            foreground_color: None,
            background_color: None,
            underline_color: None,
            attributes: Attributes::from(Attribute::Bold),
        }))?;
        write!(stdout(), " > ")?;
        stdout().queue(ResetColor)?;
        write!(stdout(), "{}", self.prompt)?;
        Ok(())
    }

    pub fn show(&self) -> Res<()> {
        stdout().queue(Clear(terminal::ClearType::CurrentLine))?;
        stdout().queue(MoveToColumn(0))?;
        self.show_pwd()?;
        self.show_input()?;
        stdout().flush()?;
        Ok(())
    }
}
