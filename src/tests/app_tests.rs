use crate::app::{App, Step};
use crate::presets::Category;

#[test]
fn app_starts_at_category_step() {
    let app = App::new();
    assert_eq!(app.step, Step::Category);
}

#[test]
fn app_starts_with_no_category_selected() {
    let app = App::new();
    assert!(app.selected_category.is_none());
}

#[test]
fn app_starts_with_first_category_highlighted() {
    let app = App::new();
    assert_eq!(app.category_state.selected(), Some(0));
}
