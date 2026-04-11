use ratatui::style::{Color, Modifier};
use crate::style::theme;

#[test]
fn selected_is_magenta_and_bold() {
    assert_eq!(theme::SELECTED.fg, Some(Color::Magenta));
    assert!(theme::SELECTED.add_modifier.contains(Modifier::BOLD));
}

#[test]
fn unselected_is_dark_gray() {
    assert_eq!(theme::UNSELECTED.fg, Some(Color::DarkGray));
}

#[test]
fn border_is_dark_gray() {
    assert_eq!(theme::BORDER.fg, Some(Color::DarkGray));
}

#[test]
fn title_is_white() {
    assert_eq!(theme::TITLE.fg, Some(Color::White));
}

#[test]
fn hint_is_dark_gray_and_dim() {
    assert_eq!(theme::HINT.fg, Some(Color::DarkGray));
    assert!(theme::HINT.add_modifier.contains(Modifier::DIM));
}
