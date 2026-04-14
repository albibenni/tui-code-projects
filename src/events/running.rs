use crossterm::event::KeyEvent;

use crate::app::{App, Step};
use crate::events::NavAction;

impl App {
    pub fn handle_running(&mut self, key: KeyEvent) {
        match NavAction::from_key(key) {
            NavAction::Confirm if self.scaffold_done => self.step = Step::Done,
            NavAction::Quit                          => self.show_quit_confirm = true,
            _                                        => {}
        }
    }
}
