use crossterm::event::KeyEvent;

use crate::app::{App, Step};
use crate::events::NavAction;
use crate::scaffold;

impl App {
    pub fn handle_confirm(&mut self, key: KeyEvent) {
        match NavAction::from_key(key) {
            NavAction::Confirm => {
                self.result_message = match scaffold::run(self) {
                    Ok(path) => format!("Project created at {path}"),
                    Err(e)   => format!("Error: {e}"),
                };
                self.step = Step::Done;
            }
            NavAction::Back => self.step = Step::Config,
            NavAction::Quit => self.should_quit = true,
            _               => {}
        }
    }
}
