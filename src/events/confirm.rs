use crossterm::event::KeyEvent;

use crate::app::{App, Step};
use crate::events::NavAction;

impl App {
    pub fn handle_confirm(&mut self, key: KeyEvent) {
        match NavAction::from_key(key) {
            NavAction::Confirm          => self.step = Step::Done,
            NavAction::Back             => self.step = Step::Config,
            NavAction::Quit             => self.should_quit = true,
            _                           => {}
        }
    }
}
