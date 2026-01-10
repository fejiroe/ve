use ratatui::termion::event::Key;
use std::io::{Result, Write};

use crate::buffer::Location;

#[derive(Debug, Default, Clone, Copy)]
pub struct Cursor {
    pub x: usize,
    pub y: usize,
}

impl From<Cursor> for Location {
    fn from(c: Cursor) -> Self {
        Location { x: c.x, y: c.y }
    }
}

impl Cursor {
    pub fn move_left(&mut self, buffer: &crate::buffer::Buffer) {
        if self.x > 0 {
            self.x -= 1;
        } else if self.y > 0 {
            self.y -= 1;
            let prev_len = buffer.line_at(self.y).len();
            self.x = prev_len.saturating_sub(1);
        }
    }
    pub fn move_right(&mut self, buffer: &crate::buffer::Buffer) {
        let line_len = buffer.line_at(self.y).len();
        if self.x + 1 < line_len {
            self.x += 1;
        } else if self.y + 1 < buffer.line_count() {
            self.y += 1;
            self.x = 0;
        }
    }
    pub fn move_up(&mut self, buffer: &crate::buffer::Buffer) {
        if self.y > 0 {
            self.y -= 1;
        }
        let line_len = buffer.line_at(self.y).len();
        if self.x > line_len {
            self.x = line_len;
        }
    }
    pub fn move_down(&mut self, buffer: &crate::buffer::Buffer) {
        let last_line = buffer.line_count().saturating_sub(1);
        if self.y < last_line {
            self.y += 1;
        }
        let line_len = buffer.line_at(self.y).len();
        if self.x > line_len {
            self.x = line_len;
        }
    }
    pub fn maybe_scroll(&self, view: &crate::view::View) -> (usize, usize) {
        let (cols, rows) = ratatui::termion::terminal_size().unwrap_or((80, 24));
        let max_cols = cols as usize;
        let max_rows = rows as usize;
        let mut new_offset_x = view.offset_x;
        if self.x < view.offset_x {
            new_offset_x = self.x;
        } else if self.x >= view.offset_x + max_cols {
            new_offset_x = self.x + 1 - max_cols;
        }
        let mut new_offset_y = view.offset_y;
        if self.y < view.offset_y {
            new_offset_y = self.y;
        } else if self.y >= view.offset_y + max_rows {
            new_offset_y = self.y + 1 - max_rows;
        }

        (new_offset_x, new_offset_y)
    }
    pub fn render_cursor<W: Write>(
        &self,
        view_offset_x: usize,
        view_offset_y: usize,
        stdout: &mut W,
    ) -> Result<()> {
        let cur_x = (self.x as i32 - view_offset_x as i32).max(0) as u16 + 1;
        let cur_y = (self.y as i32 - view_offset_y as i32).max(0) as u16 + 1;
        write!(stdout, "{}", ratatui::termion::cursor::Goto(cur_x, cur_y))?;
        stdout.flush()
    }
}

impl Cursor {
    pub fn handle_key(&mut self, key: Key, buffer: &crate::buffer::Buffer) -> (usize, usize) {
        match key {
            Key::Left => self.move_left(buffer),
            Key::Right => self.move_right(buffer),
            Key::Up => self.move_up(buffer),
            Key::Down => self.move_down(buffer),
            _ => {}
        }
        self.maybe_scroll(&crate::view::View::default())
    }
}
