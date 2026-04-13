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

fn at_done_step() -> App {
    let mut app = App::new();
    app.step = Step::Done;
    app
}

#[test]
fn enter_quits() {
    let mut app = at_done_step();
    app.handle_key(press(KeyCode::Enter));
    assert!(app.should_quit);
}

#[test]
fn q_quits() {
    let mut app = at_done_step();
    app.handle_key(press(KeyCode::Char('q')));
    assert!(app.should_quit);
}

#[test]
fn other_keys_do_nothing() {
    let mut app = at_done_step();
    app.handle_key(press(KeyCode::Char('x')));
    assert!(!app.should_quit);
    assert_eq!(app.step, Step::Done);
}
