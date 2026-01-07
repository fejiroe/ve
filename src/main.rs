use std::env;
use std::io::ErrorKind;
use std::io::{Error, Result};
use std::path::PathBuf;

use ve::Editor;

fn main() -> Result<()> {
    let mut editor = Editor::default();
    if let Some(file_name) = env::args().nth(1) {
        let path = PathBuf::from(&file_name);
        editor.open_file(&path)?;
    }
    editor.run()?;
    Ok(())
}
