use crossterm::event::{KeyCode, KeyEvent};

use crate::app::App;

impl App {
    /// Returns `true` if the quit confirm dialog consumed the key event.
    pub fn handle_quit_confirm(&mut self, key: KeyEvent) -> bool {
        if !self.show_quit_confirm {
            return false;
        }
        match key.code {
            KeyCode::Char('y') | KeyCode::Enter => self.should_quit = true,
            KeyCode::Char('n') | KeyCode::Esc => self.show_quit_confirm = false,
            _ => {}
        }
        true
    }
}
