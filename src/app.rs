use std::sync::mpsc::Receiver;

use ratatui::widgets::ListState;

use crate::config::ConfigState;
use crate::presets::{Category, Language, OptionStep, get_languages};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Step {
    Category,
    Language,
    Options,
    Config,
    Confirm,
    Running,
    Done,
}

pub struct OptionSelection {
    pub title: &'static str,
    pub choice_name: String, // Comma separated if multi
    pub choice_indices: Vec<usize>,
    pub follow_up_count: usize,
    pub is_multi: bool,
}

pub struct App {
    pub step: Step,
    pub should_quit: bool,
    pub show_quit_confirm: bool,
    // Category step
    pub category_state: ListState,
    pub selected_category: Option<Category>,
    // Language step
    pub languages: Vec<Language>,
    pub lang_state: ListState,
    // Options step
    pub option_steps: Vec<OptionStep>,
    pub option_step_index: usize,
    pub option_selections: Vec<OptionSelection>,
    pub current_multi_indices: Vec<usize>,
    pub option_list_state: ListState,
    // Config step
    pub config: ConfigState,
    // Running step
    pub scaffold_rx: Option<Receiver<String>>,
    pub output_lines: Vec<String>,
    pub scaffold_done: bool,
    // Done step
    pub result_message: String,
}

impl App {
    pub fn new() -> Self {
        let mut category_state = ListState::default();
        category_state.select(Some(0));
        let mut lang_state = ListState::default();
        lang_state.select(Some(0));
        let mut option_list_state = ListState::default();
        option_list_state.select(Some(0));
        App {
            step: Step::Category,
            should_quit: false,
            show_quit_confirm: false,
            category_state,
            selected_category: None,
            languages: get_languages(),
            lang_state,
            option_steps: Vec::new(),
            option_step_index: 0,
            option_selections: Vec::new(),
            current_multi_indices: Vec::new(),
            option_list_state,
            config: ConfigState::new(),
            scaffold_rx: None,
            output_lines: Vec::new(),
            scaffold_done: false,
            result_message: String::new(),
        }
    }

    pub fn filtered_languages(&self) -> Vec<&Language> {
        self.languages
            .iter()
            .filter(|l| Some(l.category) == self.selected_category)
            .collect()
    }

    pub fn selected_language(&self) -> Option<&Language> {
        let i = self.lang_state.selected()?;
        self.filtered_languages().into_iter().nth(i)
    }

    pub fn current_option_step(&self) -> Option<&OptionStep> {
        self.option_steps.get(self.option_step_index)
    }
}
