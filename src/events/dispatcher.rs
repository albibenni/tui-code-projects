use crossterm::event::KeyEvent;

use crate::app::{App, Step};

impl App {
    pub fn handle_key(&mut self, key: KeyEvent) {
        if self.handle_quit_confirm(key) {
            return;
        }

        match self.step {
            Step::Category => self.handle_category(key),
            Step::Language => self.handle_language(key),
            Step::Options => self.handle_options(key),
            Step::Config => self.handle_config(key),
            Step::Confirm => self.handle_confirm(key),
            Step::Running => self.handle_running(key),
            Step::Done => self.handle_done(key),
        }
    }
}
