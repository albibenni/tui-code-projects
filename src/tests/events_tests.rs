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

// --- Language step ---

fn at_language_step(category: Category) -> App {
    let mut app = App::new();
    app.selected_category = Some(category);
    app.step = Step::Language;
    app.lang_state.select(Some(0));
    app
}

#[test]
fn language_down_moves_selection_forward() {
    let mut app = at_language_step(Category::Backend);
    app.handle_key(press(KeyCode::Down));
    assert_eq!(app.lang_state.selected(), Some(1));
}

#[test]
fn language_up_does_not_go_before_first() {
    let mut app = at_language_step(Category::Backend);
    app.handle_key(press(KeyCode::Up));
    assert_eq!(app.lang_state.selected(), Some(0));
}

#[test]
fn language_down_does_not_go_past_last() {
    let mut app = at_language_step(Category::Backend);
    let last = app.filtered_languages().len() - 1;
    for _ in 0..last + 5 {
        app.handle_key(press(KeyCode::Down));
    }
    assert_eq!(app.lang_state.selected(), Some(last));
}

#[test]
fn language_enter_advances_to_preset_step() {
    let mut app = at_language_step(Category::Backend);
    app.handle_key(press(KeyCode::Enter));
    assert_eq!(app.step, Step::Preset);
    assert_eq!(app.preset_state.selected(), Some(0));
}

#[test]
fn language_esc_goes_back_to_category() {
    let mut app = at_language_step(Category::Backend);
    app.handle_key(press(KeyCode::Esc));
    assert_eq!(app.step, Step::Category);
    assert_eq!(app.lang_state.selected(), Some(0));
}

#[test]
fn language_b_goes_back_to_category() {
    let mut app = at_language_step(Category::Frontend);
    app.handle_key(press(KeyCode::Char('b')));
    assert_eq!(app.step, Step::Category);
}

#[test]
fn language_q_quits() {
    let mut app = at_language_step(Category::Backend);
    app.handle_key(press(KeyCode::Char('q')));
    assert!(app.should_quit);
}

// --- Preset step ---

fn at_preset_step(category: Category, lang_index: usize) -> App {
    let mut app = App::new();
    app.selected_category = Some(category);
    app.lang_state.select(Some(lang_index));
    app.step = Step::Preset;
    app.preset_state.select(Some(0));
    app
}

#[test]
fn preset_down_moves_selection_forward() {
    let mut app = at_preset_step(Category::Backend, 0); // Rust
    app.handle_key(press(KeyCode::Down));
    assert_eq!(app.preset_state.selected(), Some(1));
}

#[test]
fn preset_up_does_not_go_before_first() {
    let mut app = at_preset_step(Category::Backend, 0);
    app.handle_key(press(KeyCode::Up));
    assert_eq!(app.preset_state.selected(), Some(0));
}

#[test]
fn preset_down_does_not_go_past_last() {
    let mut app = at_preset_step(Category::Backend, 0);
    let last = app.selected_language().unwrap().presets.len() - 1;
    for _ in 0..last + 5 {
        app.handle_key(press(KeyCode::Down));
    }
    assert_eq!(app.preset_state.selected(), Some(last));
}

#[test]
fn preset_enter_advances_to_config_step() {
    let mut app = at_preset_step(Category::Backend, 0);
    app.handle_key(press(KeyCode::Enter));
    assert_eq!(app.step, Step::Config);
}

#[test]
fn preset_esc_goes_back_to_language() {
    let mut app = at_preset_step(Category::Backend, 0);
    app.handle_key(press(KeyCode::Esc));
    assert_eq!(app.step, Step::Language);
    assert_eq!(app.preset_state.selected(), Some(0));
}

#[test]
fn preset_b_goes_back_to_language() {
    let mut app = at_preset_step(Category::Frontend, 0);
    app.handle_key(press(KeyCode::Char('b')));
    assert_eq!(app.step, Step::Language);
}

#[test]
fn preset_q_quits() {
    let mut app = at_preset_step(Category::Backend, 0);
    app.handle_key(press(KeyCode::Char('q')));
    assert!(app.should_quit);
}
