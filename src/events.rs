use crossterm::event::{KeyCode, KeyEvent};

use crate::app::{App, Step};
use crate::presets::Category;

impl App {
    pub fn handle_key(&mut self, key: KeyEvent) {
        match self.step {
            Step::Category => self.handle_category(key),
            _ => {}
        }
    }

    fn handle_category(&mut self, key: KeyEvent) {
        let count = Category::all().len();
        match key.code {
            KeyCode::Down | KeyCode::Char('j') => {
                let next = self.category_state.selected().unwrap_or(0);
                self.category_state.select(Some((next + 1).min(count - 1)));
            }
            KeyCode::Up | KeyCode::Char('k') => {
                let prev = self.category_state.selected().unwrap_or(0);
                self.category_state.select(Some(prev.saturating_sub(1)));
            }
            KeyCode::Enter => {
                let i = self.category_state.selected().unwrap_or(0);
                self.selected_category = Some(Category::all()[i]);
                self.step = Step::Language;
            }
            KeyCode::Char('q') | KeyCode::Esc => {
                self.should_quit = true;
            }
            _ => {}
        }
    }
}
