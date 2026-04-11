use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers};

use crate::app::{App, Step};
use crate::presets::Category;

fn press(code: KeyCode) -> KeyEvent {
    KeyEvent {
        code,
        modifiers: KeyModifiers::NONE,
        kind: KeyEventKind::Press,
        state: KeyEventState::NONE,
    }
}

#[test]
fn down_moves_selection_forward() {
    let mut app = App::new();
    app.handle_key(press(KeyCode::Down));
    assert_eq!(app.category_state.selected(), Some(1));
}

#[test]
fn j_moves_selection_forward() {
    let mut app = App::new();
    app.handle_key(press(KeyCode::Char('j')));
    assert_eq!(app.category_state.selected(), Some(1));
}

#[test]
fn down_does_not_go_past_last_item() {
    let mut app = App::new();
    let last = Category::all().len() - 1;
    for _ in 0..last + 5 {
        app.handle_key(press(KeyCode::Down));
    }
    assert_eq!(app.category_state.selected(), Some(last));
}

#[test]
fn up_moves_selection_back() {
    let mut app = App::new();
    app.handle_key(press(KeyCode::Down));
    app.handle_key(press(KeyCode::Up));
    assert_eq!(app.category_state.selected(), Some(0));
}

#[test]
fn k_moves_selection_back() {
    let mut app = App::new();
    app.handle_key(press(KeyCode::Down));
    app.handle_key(press(KeyCode::Char('k')));
    assert_eq!(app.category_state.selected(), Some(0));
}

#[test]
fn up_does_not_go_before_first_item() {
    let mut app = App::new();
    app.handle_key(press(KeyCode::Up));
    assert_eq!(app.category_state.selected(), Some(0));
}

#[test]
fn enter_sets_selected_category_and_advances_step() {
    let mut app = App::new();
    app.handle_key(press(KeyCode::Enter));
    assert_eq!(app.selected_category, Some(Category::Backend));
    assert_eq!(app.step, Step::Language);
}

#[test]
fn enter_on_second_item_selects_frontend() {
    let mut app = App::new();
    app.handle_key(press(KeyCode::Down));
    app.handle_key(press(KeyCode::Enter));
    assert_eq!(app.selected_category, Some(Category::Frontend));
    assert_eq!(app.step, Step::Language);
}

#[test]
fn q_quits() {
    let mut app = App::new();
    app.handle_key(press(KeyCode::Char('q')));
    assert!(app.should_quit);
}

#[test]
fn esc_quits() {
    let mut app = App::new();
    app.handle_key(press(KeyCode::Esc));
    assert!(app.should_quit);
}
