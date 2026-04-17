use crossterm::event::KeyEvent;

use crate::app::App;
use crate::events::NavAction;

impl App {
    pub fn handle_done(&mut self, key: KeyEvent) {
        match NavAction::from_key(key) {
            NavAction::Confirm | NavAction::Quit => self.show_quit_confirm = true,
            _ => {}
        }
    }
}
