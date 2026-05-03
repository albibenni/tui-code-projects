mod app;
mod config;
mod events;
mod presets;
mod scaffold;
mod style;
#[cfg(test)]
mod tests;

use std::sync::mpsc::TryRecvError;
use std::time::Duration;

use app::{App, Step};
use crossterm::event::{self, Event, KeyEventKind};
use presets::ui;
use scaffold::cleanup;
use std::io;

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    
    // Ensure terminal is restored even on panic
    let original_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        ratatui::restore();
        original_hook(panic_info);
    }));

    let result = run(&mut terminal);
    ratatui::restore();
    result
}

fn run(terminal: &mut ratatui::DefaultTerminal) -> io::Result<()> {
    let mut app = App::new();

    while !app.should_quit {
        terminal.draw(|frame| ui::draw(frame, &mut app))?;

        if event::poll(Duration::from_millis(16))?
            && let Event::Key(key) = event::read()?
            && key.kind == KeyEventKind::Press
        {
            app.handle_key(key);
        }

        drain_scaffold(&mut app);
    }

    scaffold::INTERRUPTED.store(true, std::sync::atomic::Ordering::SeqCst);

    if app.step == Step::Running && !app.scaffold_done {
        if let Some(handle) = app.scaffold_handle.take() {
            let _ = handle.join();
        }
        cleanup(&app.config.project_path, &app.config.project_name);
        println!(
            "\nProject creation interrupted for '{}'. Cleaned up partial files.",
            app.config.project_name
        );
    }

    Ok(())
}

fn drain_scaffold(app: &mut App) {
    let Some(rx) = &app.scaffold_rx else { return };

    loop {
        match rx.try_recv() {
            Ok(line) => {
                app.output_lines.push(line);
            }
            Err(TryRecvError::Empty) => break,
            Err(TryRecvError::Disconnected) => {
                app.scaffold_done = true;
                app.result_message = app
                    .output_lines
                    .last()
                    .cloned()
                    .unwrap_or_else(|| "Done".to_string());
                app.scaffold_rx = None;
                break;
            }
        }
    }
}
