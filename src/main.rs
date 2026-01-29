use std::env;
use std::io::Result;
use std::path::PathBuf;
use ratatui::termion::{
    cursor,
    raw::{IntoRawMode},
    screen::{ToAlternateScreen, ToMainScreen},
};
use std::io::{Write, stdout};
use ve::Editor;

fn main() -> Result<()> {
    let mut stdout = stdout().into_raw_mode()?;
    write!(stdout, "{}", ToAlternateScreen)?;
    let mut editor = Editor::default();
    if let Some(file_name) = env::args().nth(1) {
        let path = PathBuf::from(&file_name);
        editor.open_file(&path)?;
    }
    editor.run(&mut stdout)?;
    stdout.flush()?;
    let _ = write!(stdout, "{}", ToMainScreen);
    let _ = write!(stdout, "{}", cursor::Show);
    Ok(())
}
