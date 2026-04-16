use crossterm::event::KeyEvent;

use crate::app::{App, Step};
use crate::events::{Direction, NavAction, move_list_selection};

impl App {
    pub fn handle_language(&mut self, key: KeyEvent) {
        let count = self.filtered_languages().len();
        match NavAction::from_key(key) {
            NavAction::Down => move_list_selection(&mut self.lang_state, count, Direction::Down),
            NavAction::Up => move_list_selection(&mut self.lang_state, count, Direction::Up),
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
