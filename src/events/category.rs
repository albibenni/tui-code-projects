use crossterm::event::KeyEvent;

use crate::app::{App, Step};
use crate::events::NavAction;
use crate::presets::Category;

impl App {
    pub fn handle_category(&mut self, key: KeyEvent) {
        let count = Category::all().len();
        match NavAction::from_key(key) {
            NavAction::Down => {
                let next = self.category_state.selected().unwrap_or(0);
                self.category_state.select(Some((next + 1).min(count - 1)));
            }
            NavAction::Up => {
                let prev = self.category_state.selected().unwrap_or(0);
                self.category_state.select(Some(prev.saturating_sub(1)));
            }
            NavAction::Confirm => {
                let i = self.category_state.selected().unwrap_or(0);
                self.selected_category = Some(Category::all()[i]);
                self.lang_state.select(Some(0));
                self.step = Step::Language;
            }
            NavAction::Back | NavAction::Quit => {
                self.show_quit_confirm = true;
            }
            NavAction::Other => {}
        }
    }
}
