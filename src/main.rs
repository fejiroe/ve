fn main() -> std::io::Result<()> {
    ratatui::run(|mut terminal| {
        loop {
            terminal.draw(render)?;
            if should_quit()? {
                break Ok(());
            }
        }
    })
}

fn render(frame: &mut ratatui::Frame) {
    // ...
}

fn should_quit() -> std::io::Result<bool> {
    // ...
}