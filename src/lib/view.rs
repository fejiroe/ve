use crate::buffer::Buffer;
use ratatui::termion::terminal_size;
use std::io::{Result, Write};

#[derive(Default)]
pub struct View {
    pub needs_update: bool, // still does nothing
    pub offset_y: usize,
    pub offset_x: usize,
}
impl View {
    pub fn render<W: Write>(&self, stdout: &mut W, buffer: &Buffer) -> Result<()> {
        write!(
            stdout,
            "{}{}",
            ratatui::termion::clear::All,
            ratatui::termion::cursor::Goto(1, 1)
        )?;
        let (cols, rows) = terminal_size().unwrap_or((80, 24));
        let max_cols = cols as usize;
        let max_rows = rows as usize;
        let start_line = self.offset_y;
        let end_line = usize::min(start_line + max_rows, buffer.line_count());
        for line in &buffer.lines[start_line..end_line] {
            let start_byte = *line.graphemes.get(self.offset_x).unwrap_or(&line.raw.len());
            let end_byte = usize::min(start_byte + max_cols, line.raw.len());
            let visible = &line.raw[start_byte..end_byte];
            write!(stdout, "{}\r\n", visible)?;
        }
        stdout.flush()
    }
}
