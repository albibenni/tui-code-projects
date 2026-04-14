use std::sync::mpsc;
use std::thread;

use crossterm::event::KeyEvent;

use crate::app::{App, Step};
use crate::events::NavAction;
use crate::scaffold::{ScaffoldParams, run_threaded};

impl App {
    pub fn handle_confirm(&mut self, key: KeyEvent) {
        match NavAction::from_key(key) {
            NavAction::Confirm => {
                let params = ScaffoldParams {
                    project_path: self.config.project_path.clone(),
                    project_name: self.config.project_name.clone(),
                    language_name: self
                        .selected_language()
                        .map(|l| l.name.to_string())
                        .unwrap_or_default(),
                    selections: self
                        .option_selections
                        .iter()
                        .map(|s| (s.title.to_string(), s.choice_name.to_string()))
                        .collect(),
                };

                let (tx, rx) = mpsc::channel::<String>();
                self.scaffold_rx = Some(rx);
                self.step = Step::Running;

                thread::spawn(move || run_threaded(params, tx));
            }
            NavAction::Back => self.step = Step::Config,
            NavAction::Quit => self.should_quit = true,
            _               => {}
        }
    }
}
