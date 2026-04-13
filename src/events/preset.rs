use crossterm::event::KeyEvent;

use crate::app::{App, Step};
use crate::events::NavAction;

impl App {
    pub fn handle_preset(&mut self, key: KeyEvent) {
        let count = self
            .selected_language()
            .map(|l| l.presets.len())
            .unwrap_or(0);
        match NavAction::from_key(key) {
            NavAction::Down => {
                let next = self.preset_state.selected().unwrap_or(0);
                self.preset_state.select(Some((next + 1).min(count - 1)));
            }
            NavAction::Up => {
                let prev = self.preset_state.selected().unwrap_or(0);
                self.preset_state.select(Some(prev.saturating_sub(1)));
            }
            NavAction::Confirm => {
                self.step = Step::Config;
            }
            NavAction::Back => {
                self.preset_state.select(Some(0));
                self.step = Step::Language;
            }
            NavAction::Quit => {
                self.should_quit = true;
            }
            NavAction::Other => {}
        }
    }
}
