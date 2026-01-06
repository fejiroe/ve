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

#[derive(Default, Clone, Copy)]
struct Location {
    x: usize,
    y: usize,
}

struct Editor {
    mode: Mode,
    location: Location,
}

impl Editor {
    fn default() -> Self {
        Self {
            mode: Mode::Normal,
            location: Location { x: 0, y: 0 },
        }
    }
    fn set_mode(&mut self, mode: Mode) {
        self.mode = mode;
    }
    fn udate_cursor(&self, stdout: &mut std::io::Stdout) -> Result<()> {
        write!(
            stdout,
            "{}",
            ratatui::termion::cursor::Goto(self.location.x as u16 + 1, self.location.y as u16 + 1)
        )?;
        stdout.flush()?;
        Ok(())
    }
    fn move_left(&mut self, stdout: &mut std::io::Stdout) -> Result<()> {
        self.location.x -= 1;
        self.udate_cursor(stdout)?;
        Ok(())
    }
    fn move_right(&mut self, stdout: &mut std::io::Stdout) -> Result<()> {
        self.location.x += 1;
        self.udate_cursor(stdout)?;
        Ok(())
    }
    fn move_up(&mut self, stdout: &mut std::io::Stdout) -> Result<()> {
        self.location.y -= 1;
        self.udate_cursor(stdout)?;
        Ok(())
    }
    fn move_down(&mut self, stdout: &mut std::io::Stdout) -> Result<()> {
        self.location.y += 1;
        self.udate_cursor(stdout)?;
        Ok(())
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
                    Key::Left => self.move_left(&mut stdout)?,
                    Key::Right => self.move_right(&mut stdout)?,
                    Key::Up => self.move_up(&mut stdout)?,
                    Key::Down => self.move_down(&mut stdout)?,
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
                        self.move_left(&mut stdout)?;
                        write!(stdout, "\x08 \x08")?;
                    }
                    Key::Esc => {
                        self.set_mode(Mode::Normal);
                    }
                    Key::Char('i') => self.set_mode(Mode::Insert),
                    Key::Left => self.move_left(&mut stdout)?,
                    Key::Right => self.move_right(&mut stdout)?,
                    Key::Up => self.move_up(&mut stdout)?,
                    Key::Down => self.move_down(&mut stdout)?,
                    _ => {}
                },
            }
            stdout.flush().unwrap();
        }
        Ok(())
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
