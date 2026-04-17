use crossterm::event::KeyEvent;

use crate::app::{App, Step};
use crate::events::{Direction, NavAction, move_list_selection};
use crate::presets::Category;

impl App {
    pub fn handle_category(&mut self, key: KeyEvent) {
        let count = Category::all().len();
        match NavAction::from_key(key) {
            NavAction::Down => {
                move_list_selection(&mut self.category_state, count, Direction::Down)
            }
            NavAction::Up => move_list_selection(&mut self.category_state, count, Direction::Up),
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
