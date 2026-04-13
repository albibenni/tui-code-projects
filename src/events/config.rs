use crossterm::event::{KeyCode, KeyEvent};

use crate::app::{App, Step};

impl App {
    pub fn handle_config(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char(c) => {
                self.config.active_value_mut().push(c);
                self.config.error_message = None;
            }
            KeyCode::Backspace => {
                self.config.active_value_mut().pop();
            }
            KeyCode::Tab => {
                self.config.toggle_field();
            }
            KeyCode::Enter => {
                if self.config.project_name.trim().is_empty() {
                    self.config.error_message = Some("Project name cannot be empty".into());
                } else {
                    self.config.error_message = None;
                    self.step = Step::Confirm;
                }
            }
            KeyCode::Esc => {
                self.config.error_message = None;
                self.step = Step::Options;
            }
            _ => {}
        }
    }
}
