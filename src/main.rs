use ratatui::termion::event::Key;
use ratatui::termion::input::TermRead;
use ratatui::termion::raw::IntoRawMode;
use ratatui::termion::terminal_size;
use std::env;
use std::io::{Error, Read, Result, Write, stdin, stdout};

enum Mode {
    Normal,
    Insert,
}

struct Editor {
    mode: Mode,
}

impl Editor {
    fn default() -> Self {
        Self { mode: Mode::Normal }
    }
    fn run(&mut self) -> Result<()> {
        // let mut size = terminal_size().unwrap();
        let stdin = stdin();
        let mut stdout = stdout().into_raw_mode()?;
        write!(
            stdout,
            "{}{}",
            ratatui::termion::clear::All,
            ratatui::termion::cursor::Goto(1, 1)
        )
        .unwrap();
        stdout.flush().unwrap();
        for k in stdin.keys() {
            let key = k.unwrap();
            match self.mode {
                Mode::Normal => match key {
                    // Key::Char('a') => ,
                    Key::Char('i') => self.set_mode(Mode::Insert),
                    // Key::Char('x') => ,
                    // Key::Char('s') => ,
                    // Key::Char('r') => ,
                    // Key::Char('v') => ,
                    Key::Ctrl(c) => match c {
                        'q' => break,
                        _ => {}
                    },
                    _ => {}
                },
                Mode::Insert => match key {
                    Key::Char(c) => {
                        write!(stdout, "{}", c)?;
                    }
                    Key::Backspace => {
                        write!(stdout, "\x08 \x08")?;
                    }
                    Key::Esc => {
                        self.set_mode(Mode::Normal);
                    }
                    _ => {}
                },
            }
            stdout.flush().unwrap();
        }
        Ok(())
    }
    fn set_mode(&mut self, mode: Mode) {
        self.mode = mode;
    }
}

fn main() -> Result<()> {
    Editor::default().run();
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_something() {}
}
