use crossterm::event::KeyEvent;

use crate::app::{App, Step};
use crate::events::NavAction;

impl App {
    pub fn handle_language(&mut self, key: KeyEvent) {
        let count = self.filtered_languages().len();
        match NavAction::from_key(key) {
            NavAction::Down => {
                let next = self.lang_state.selected().unwrap_or(0);
                self.lang_state.select(Some((next + 1).min(count - 1)));
            }
            NavAction::Up => {
                let prev = self.lang_state.selected().unwrap_or(0);
                self.lang_state.select(Some(prev.saturating_sub(1)));
            }
            NavAction::Confirm => {
                if let Some(lang) = self.selected_language() {
                    self.option_steps = lang.steps.clone();
                    self.option_step_index = 0;
                    self.option_selections.clear();
                    self.option_list_state.select(Some(0));
                    self.step = Step::Options;
                }
            }
            NavAction::Back => {
                self.lang_state.select(Some(0));
                self.step = Step::Category;
            }
            NavAction::Quit => {
                self.show_quit_confirm = true;
            }
            NavAction::Other => {}
        }
    }
}
