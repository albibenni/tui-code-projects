use crossterm::event::{KeyCode, KeyEvent};

use crate::app::{App, Step};
use crate::config::validate_project_name;

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
                let lang_name = self.selected_language().map(|l| l.name);
                match validate_project_name(&self.config.project_name, lang_name) {
                    Ok(()) => {
                        self.config.error_message = None;
                        self.step = Step::Confirm;
                    }
                    Err(message) => {
                        self.config.error_message = Some(message.into());
                    }
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
