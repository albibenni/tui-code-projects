use crossterm::event::KeyEvent;
use ratatui::widgets::ListState;

use crate::presets::{Category, Language, get_languages};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Step {
    Category,
    Language,
    Preset,
    Config,
    Confirm,
    Done,
}

pub struct App {
    pub step: Step,
    pub should_quit: bool,
    pub category_state: ListState,
    pub selected_category: Option<Category>,
    pub languages: Vec<Language>,
    pub lang_state: ListState,
    pub preset_state: ListState,
    pub project_name: String,
    pub project_path: String,
    pub result_message: String,
    pub error_message: Option<String>,
}

impl App {
    pub fn new() -> Self {
        let mut category_state = ListState::default();
        category_state.select(Some(0));
        let mut lang_state = ListState::default();
        lang_state.select(Some(0));
        let mut preset_state = ListState::default();
        preset_state.select(Some(0));
        App {
            step: Step::Category,
            should_quit: false,
            category_state,
            selected_category: None,
            languages: get_languages(),
            lang_state,
            preset_state,
            project_name: String::new(),
            project_path: String::from("./"),
            result_message: String::new(),
            error_message: None,
        }
    }

    pub fn handle_key(&mut self, _key: KeyEvent) {
        todo!()
    }
}
