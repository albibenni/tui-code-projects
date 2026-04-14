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

// ── Category step ─────────────────────────────────────────────────────────────

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
fn q_shows_quit_confirm() {
    let mut app = App::new();
    app.handle_key(press(KeyCode::Char('q')));
    assert!(app.show_quit_confirm);
}

#[test]
fn esc_shows_quit_confirm() {
    let mut app = App::new();
    app.handle_key(press(KeyCode::Esc));
    assert!(app.show_quit_confirm);
}

// ── Language step ─────────────────────────────────────────────────────────────

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
fn language_enter_advances_to_options_step() {
    let mut app = at_language_step(Category::Backend);
    app.handle_key(press(KeyCode::Enter));
    assert_eq!(app.step, Step::Options);
    assert_eq!(app.option_list_state.selected(), Some(0));
    assert!(!app.option_steps.is_empty());
}

#[test]
fn language_enter_loads_option_steps_for_language() {
    let mut app = at_language_step(Category::Backend);
    app.handle_key(press(KeyCode::Enter)); // selects Rust
    assert_eq!(app.option_steps[0].title, "Project Type");
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
fn language_q_shows_quit_confirm() {
    let mut app = at_language_step(Category::Backend);
    app.handle_key(press(KeyCode::Char('q')));
    assert!(app.show_quit_confirm);
}

// ── Options step ──────────────────────────────────────────────────────────────

fn at_options_step(category: Category, lang_index: usize) -> App {
    let mut app = App::new();
    app.selected_category = Some(category);
    app.lang_state.select(Some(lang_index));
    let steps = app.selected_language().unwrap().steps.clone();
    app.option_steps = steps;
    app.option_step_index = 0;
    app.option_list_state.select(Some(0));
    app.step = Step::Options;
    app
}

#[test]
fn options_down_moves_selection_forward() {
    let mut app = at_options_step(Category::Backend, 0); // Rust
    app.handle_key(press(KeyCode::Down));
    assert_eq!(app.option_list_state.selected(), Some(1));
}

#[test]
fn options_up_does_not_go_before_first() {
    let mut app = at_options_step(Category::Backend, 0);
    app.handle_key(press(KeyCode::Up));
    assert_eq!(app.option_list_state.selected(), Some(0));
}

#[test]
fn options_down_does_not_go_past_last() {
    let mut app = at_options_step(Category::Backend, 0);
    let last = app.current_option_step().unwrap().choices.len() - 1;
    for _ in 0..last + 5 {
        app.handle_key(press(KeyCode::Down));
    }
    assert_eq!(app.option_list_state.selected(), Some(last));
}

#[test]
fn options_enter_on_leaf_advances_to_config() {
    let mut app = at_options_step(Category::Backend, 0); // Rust — Binary has no follow_up
    app.option_list_state.select(Some(0)); // Binary
    app.handle_key(press(KeyCode::Enter));
    assert_eq!(app.step, Step::Config);
}

#[test]
fn options_enter_on_choice_with_follow_up_stays_in_options() {
    let mut app = at_options_step(Category::Backend, 0); // Rust
    app.option_list_state.select(Some(2)); // Web API — has follow_up Framework step
    app.handle_key(press(KeyCode::Enter));
    assert_eq!(app.step, Step::Options);
    assert_eq!(app.option_steps.len(), 2);
    assert_eq!(app.option_step_index, 1);
    assert_eq!(app.option_steps[1].title, "Framework");
}

#[test]
fn options_back_from_first_step_goes_to_language() {
    let mut app = at_options_step(Category::Backend, 0);
    app.handle_key(press(KeyCode::Esc));
    assert_eq!(app.step, Step::Language);
    assert!(app.option_steps.is_empty());
}

#[test]
fn options_back_restores_previous_selection_and_removes_follow_up() {
    let mut app = at_options_step(Category::Backend, 0); // Rust
    app.option_list_state.select(Some(2)); // Web API
    app.handle_key(press(KeyCode::Enter)); // now at Framework step
    app.handle_key(press(KeyCode::Esc));   // go back
    assert_eq!(app.option_step_index, 0);
    assert_eq!(app.option_list_state.selected(), Some(2)); // restored
    assert_eq!(app.option_steps.len(), 1); // follow_up removed
}

#[test]
fn options_q_shows_quit_confirm() {
    let mut app = at_options_step(Category::Backend, 0);
    app.handle_key(press(KeyCode::Char('q')));
    assert!(app.show_quit_confirm);
}
