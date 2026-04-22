use crossterm::event::{KeyCode, KeyEvent};

pub enum NavAction {
    Up,
    Down,
    Confirm,
    Toggle,
    Back,
    Quit,
    Other,
}

impl NavAction {
    pub fn from_key(key: KeyEvent) -> Self {
        match key.code {
            KeyCode::Up | KeyCode::Char('k') => NavAction::Up,
            KeyCode::Down | KeyCode::Char('j') => NavAction::Down,
            KeyCode::Char(' ') => NavAction::Toggle,
            KeyCode::Enter => NavAction::Confirm,
            KeyCode::Esc | KeyCode::Char('b') => NavAction::Back,
            KeyCode::Char('q') => NavAction::Quit,
            _ => NavAction::Other,
        }
    }
}
