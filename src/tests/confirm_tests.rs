use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers};

use crate::app::{App, Step};

fn press(code: KeyCode) -> KeyEvent {
    KeyEvent {
        code,
        modifiers: KeyModifiers::NONE,
        kind: KeyEventKind::Press,
        state: KeyEventState::NONE,
    }
}

fn at_confirm_step() -> App {
    let mut app = App::new();
    app.step = Step::Confirm;
    app
}

#[test]
fn enter_advances_to_running() {
    let mut app = at_confirm_step();
    app.handle_key(press(KeyCode::Enter));
    assert_eq!(app.step, Step::Running);
}

#[test]
fn esc_goes_back_to_config() {
    let mut app = at_confirm_step();
    app.handle_key(press(KeyCode::Esc));
    assert_eq!(app.step, Step::Config);
}

#[test]
fn b_goes_back_to_config() {
    let mut app = at_confirm_step();
    app.handle_key(press(KeyCode::Char('b')));
    assert_eq!(app.step, Step::Config);
}

#[test]
fn q_shows_quit_confirm() {
    let mut app = at_confirm_step();
    app.handle_key(press(KeyCode::Char('q')));
    assert!(app.show_quit_confirm);
}

#[test]
fn other_keys_do_nothing() {
    let mut app = at_confirm_step();
    app.handle_key(press(KeyCode::Char('x')));
    assert_eq!(app.step, Step::Confirm);
    assert!(!app.should_quit);
}
