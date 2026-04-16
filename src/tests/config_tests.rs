use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers};

use crate::app::{App, Step};
use crate::config::{ConfigField, validate_project_name};

fn press(code: KeyCode) -> KeyEvent {
    KeyEvent {
        code,
        modifiers: KeyModifiers::NONE,
        kind: KeyEventKind::Press,
        state: KeyEventState::NONE,
    }
}

fn at_config_step() -> App {
    let mut app = App::new();
    app.step = Step::Config;
    app
}

#[test]
fn config_starts_with_name_field_active() {
    let app = at_config_step();
    assert_eq!(app.config.active_field, ConfigField::Name);
}

#[test]
fn config_starts_with_empty_name() {
    let app = at_config_step();
    assert!(app.config.project_name.is_empty());
}

#[test]
fn config_starts_with_default_path() {
    let app = at_config_step();
    assert_eq!(app.config.project_path, "./");
}

#[test]
fn typing_appends_to_name_field() {
    let mut app = at_config_step();
    app.handle_key(press(KeyCode::Char('m')));
    app.handle_key(press(KeyCode::Char('y')));
    app.handle_key(press(KeyCode::Char('-')));
    app.handle_key(press(KeyCode::Char('a')));
    app.handle_key(press(KeyCode::Char('p')));
    app.handle_key(press(KeyCode::Char('p')));
    assert_eq!(app.config.project_name, "my-app");
}

#[test]
fn backspace_removes_last_char_from_name() {
    let mut app = at_config_step();
    app.handle_key(press(KeyCode::Char('m')));
    app.handle_key(press(KeyCode::Char('y')));
    app.handle_key(press(KeyCode::Backspace));
    assert_eq!(app.config.project_name, "m");
}

#[test]
fn backspace_on_empty_field_does_nothing() {
    let mut app = at_config_step();
    app.handle_key(press(KeyCode::Backspace));
    assert!(app.config.project_name.is_empty());
}

#[test]
fn tab_switches_to_path_field() {
    let mut app = at_config_step();
    app.handle_key(press(KeyCode::Tab));
    assert_eq!(app.config.active_field, ConfigField::Path);
}

#[test]
fn tab_twice_switches_back_to_name_field() {
    let mut app = at_config_step();
    app.handle_key(press(KeyCode::Tab));
    app.handle_key(press(KeyCode::Tab));
    assert_eq!(app.config.active_field, ConfigField::Name);
}

#[test]
fn typing_after_tab_appends_to_path_field() {
    let mut app = at_config_step();
    app.handle_key(press(KeyCode::Tab));
    app.handle_key(press(KeyCode::Char('/')));
    app.handle_key(press(KeyCode::Char('t')));
    app.handle_key(press(KeyCode::Char('m')));
    app.handle_key(press(KeyCode::Char('p')));
    assert_eq!(app.config.project_path, ".//tmp");
}

#[test]
fn enter_with_empty_name_sets_error() {
    let mut app = at_config_step();
    app.handle_key(press(KeyCode::Enter));
    assert!(app.config.error_message.is_some());
    assert_eq!(app.step, Step::Config);
}

#[test]
fn enter_with_valid_name_advances_to_confirm() {
    let mut app = at_config_step();
    app.handle_key(press(KeyCode::Char('m')));
    app.handle_key(press(KeyCode::Char('y')));
    app.handle_key(press(KeyCode::Enter));
    assert_eq!(app.step, Step::Confirm);
    assert!(app.config.error_message.is_none());
}

#[test]
fn typing_clears_error_message() {
    let mut app = at_config_step();
    app.handle_key(press(KeyCode::Enter)); // trigger error
    app.handle_key(press(KeyCode::Char('a')));
    assert!(app.config.error_message.is_none());
}

#[test]
fn esc_goes_back_to_options() {
    let mut app = at_config_step();
    app.handle_key(press(KeyCode::Esc));
    assert_eq!(app.step, Step::Options);
}

#[test]
fn q_types_q_does_not_quit() {
    let mut app = at_config_step();
    app.handle_key(press(KeyCode::Char('q')));
    assert!(!app.should_quit);
    assert_eq!(app.config.project_name, "q");
}

#[test]
fn validate_project_name_rejects_path_separator() {
    assert!(validate_project_name("my/app").is_err());
}

#[test]
fn validate_project_name_rejects_parent_dir() {
    assert!(validate_project_name("..").is_err());
}

#[test]
fn validate_project_name_rejects_absolute_path() {
    assert!(validate_project_name("/tmp/app").is_err());
}
