mod app;
mod config;
mod events;
mod presets;
mod scaffold;
mod style;
#[cfg(test)]
mod tests;

use app::App;
use crossterm::event::{self, Event, KeyEventKind};
use presets::ui;
use std::io;

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let result = run(&mut terminal);
    ratatui::restore();
    result
}

fn run(terminal: &mut ratatui::DefaultTerminal) -> io::Result<()> {
    let mut app = App::new();
    while !app.should_quit {
        terminal.draw(|frame| ui::draw(frame, &mut app))?;
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                app.handle_key(key);
            }
        }
    }
    Ok(())
}
