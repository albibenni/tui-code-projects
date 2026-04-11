use crossterm::event::{KeyCode, KeyEvent};

use crate::app::{App, Step};
use crate::presets::Category;

enum NavAction {
    Up,
    Down,
    Confirm,
    Back,
    Quit,
    Other,
}

impl NavAction {
    fn from_key(key: KeyEvent) -> Self {
        match key.code {
            KeyCode::Up | KeyCode::Char('k')   => NavAction::Up,
            KeyCode::Down | KeyCode::Char('j') => NavAction::Down,
            KeyCode::Enter                     => NavAction::Confirm,
            KeyCode::Esc | KeyCode::Char('b')  => NavAction::Back,
            KeyCode::Char('q')                 => NavAction::Quit,
            _                                  => NavAction::Other,
        }
    }
}

impl App {
    pub fn handle_key(&mut self, key: KeyEvent) {
        match self.step {
            Step::Category => self.handle_category(key),
            Step::Language => self.handle_language(key),
            _ => {}
        }
    }

    fn handle_category(&mut self, key: KeyEvent) {
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
                self.should_quit = true;
            }
            NavAction::Other => {}
        }
    }

    fn handle_language(&mut self, key: KeyEvent) {
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
                self.preset_state.select(Some(0));
                self.step = Step::Preset;
            }
            NavAction::Back => {
                self.lang_state.select(Some(0));
                self.step = Step::Category;
            }
            NavAction::Quit => {
                self.should_quit = true;
            }
            NavAction::Other => {}
        }
    }
}
