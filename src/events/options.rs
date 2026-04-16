use crossterm::event::KeyEvent;

use crate::app::{App, OptionSelection, Step};
use crate::events::{Direction, NavAction, move_list_selection};

impl App {
    pub fn handle_options(&mut self, key: KeyEvent) {
        let count = self
            .current_option_step()
            .map(|s| s.choices.len())
            .unwrap_or(0);

        match NavAction::from_key(key) {
            NavAction::Down => move_list_selection(&mut self.option_list_state, count, Direction::Down),
            NavAction::Up => move_list_selection(&mut self.option_list_state, count, Direction::Up),
            NavAction::Confirm => {
                let choice_idx = self.option_list_state.selected().unwrap_or(0);
                // Extract data before mutating self
                let (step_title, choice_name, follow_up) =
                    match self.option_steps.get(self.option_step_index) {
                        Some(step) => match step.choices.get(choice_idx) {
                            Some(choice) => (step.title, choice.name, choice.follow_up.clone()),
                            None => return,
                        },
                        None => return,
                    };
                let follow_up_count = follow_up.len();
                self.option_selections.push(OptionSelection {
                    title: step_title,
                    choice_name,
                    choice_index: choice_idx,
                    follow_up_count,
                });
                for step in follow_up {
                    self.option_steps.push(step);
                }
                self.option_step_index += 1;
                if self.option_step_index >= self.option_steps.len() {
                    self.step = Step::Config;
                } else {
                    self.option_list_state.select(Some(0));
                }
            }
            NavAction::Back => {
                if self.option_step_index == 0 {
                    self.option_steps.clear();
                    self.option_selections.clear();
                    self.step = Step::Language;
                } else if let Some(sel) = self.option_selections.pop() {
                    let new_len = self.option_steps.len() - sel.follow_up_count;
                    self.option_steps.truncate(new_len);
                    self.option_step_index -= 1;
                    self.option_list_state.select(Some(sel.choice_index));
                }
            }
            NavAction::Quit => {
                self.show_quit_confirm = true;
            }
            NavAction::Other => {}
        }
    }
}
