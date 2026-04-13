mod category;
mod config;
mod language;
mod options;

use crossterm::event::{KeyCode, KeyEvent};

use crate::app::{App, Step};

pub enum NavAction {
    Up,
    Down,
    Confirm,
    Back,
    Quit,
    Other,
}

impl NavAction {
    pub fn from_key(key: KeyEvent) -> Self {
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
            Step::Options  => self.handle_options(key),
            Step::Config   => self.handle_config(key),
            _              => {}
        }
    }
}
