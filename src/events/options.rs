use crossterm::event::KeyEvent;

use crate::app::{App, OptionSelection, Step};
use crate::events::{Direction, NavAction, move_list_selection};
use crate::presets::OptionStep;

impl App {
    pub fn handle_options(&mut self, key: KeyEvent) {
        let (count, is_multi) = self
            .current_option_step()
            .map(|s| (s.choices.len(), s.is_multi))
            .unwrap_or((0, false));

        match NavAction::from_key(key) {
            NavAction::Down => {
                move_list_selection(&mut self.option_list_state, count, Direction::Down)
            }
            NavAction::Up => move_list_selection(&mut self.option_list_state, count, Direction::Up),
            NavAction::Toggle => {
                if is_multi {
                    if let Some(idx) = self.option_list_state.selected() {
                        if let Some(pos) = self.current_multi_indices.iter().position(|&i| i == idx) {
                            self.current_multi_indices.remove(pos);
                        } else {
                            self.current_multi_indices.push(idx);
                        }
                    }
                }
            }
            NavAction::Confirm => {
                if is_multi {
                    // Multi-select confirm: finalize selections and move to next step
                    let step = match self.option_steps.get(self.option_step_index) {
                        Some(s) => s,
                        None => return,
                    };

                    let mut names = Vec::new();
                    let mut follow_ups = Vec::new();
                    
                    // Sort indices to keep a consistent order
                    self.current_multi_indices.sort_unstable();
                    
                    for &idx in &self.current_multi_indices {
                        if let Some(choice) = step.choices.get(idx) {
                            names.push(choice.name);
                            for f in &choice.follow_up {
                                // Only add follow-up if it's not already in the follow-up list
                                if !follow_ups.iter().any(|existing: &OptionStep| existing.title == f.title) {
                                    follow_ups.push(f.clone());
                                }
                            }
                        }
                    }

                    let choice_name = if names.is_empty() {
                        "None".to_string()
                    } else {
                        names.join(", ")
                    };

                    self.option_selections.push(OptionSelection {
                        title: step.title,
                        choice_name,
                        choice_indices: self.current_multi_indices.clone(),
                        follow_up_count: follow_ups.len(),
                        is_multi: true,
                    });

                    for f in follow_ups {
                        // Also check if the step is already in self.option_steps after current index
                        if !self.option_steps[self.option_step_index + 1..].iter().any(|s| s.title == f.title) {
                            self.option_steps.push(f);
                        }
                    }

                    self.current_multi_indices.clear();
                    self.option_step_index += 1;
                } else {
                    // Single-select confirm (original logic)
                    let choice_idx = self.option_list_state.selected().unwrap_or(0);
                    let (step_title, choice_name, follow_up) =
                        match self.option_steps.get(self.option_step_index) {
                            Some(step) => match step.choices.get(choice_idx) {
                                Some(choice) => (step.title, choice.name, choice.follow_up.clone()),
                                None => return,
                            },
                            None => return,
                        };
                    
                    self.option_selections.push(OptionSelection {
                        title: step_title,
                        choice_name: choice_name.to_string(),
                        choice_indices: vec![choice_idx],
                        follow_up_count: follow_up.len(),
                        is_multi: false,
                    });
                    
                    for step in follow_up {
                        self.option_steps.push(step);
                    }
                    self.option_step_index += 1;
                }

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
                    self.current_multi_indices.clear();
                    self.step = Step::Language;
                } else if let Some(sel) = self.option_selections.pop() {
                    let new_len = self.option_steps.len() - sel.follow_up_count;
                    self.option_steps.truncate(new_len);
                    self.option_step_index -= 1;
                    
                    if sel.is_multi {
                        self.current_multi_indices = sel.choice_indices;
                        self.option_list_state.select(Some(0));
                    } else {
                        self.option_list_state.select(sel.choice_indices.first().copied());
                    }
                }
            }
            NavAction::Quit => {
                self.show_quit_confirm = true;
            }
            NavAction::Other => {}
        }
    }
}
