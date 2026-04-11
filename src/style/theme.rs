use ratatui::style::{Color, Modifier, Style};

pub const SELECTED: Style = Style::new()
    .fg(Color::Magenta)
    .add_modifier(Modifier::BOLD);

pub const UNSELECTED: Style = Style::new()
    .fg(Color::DarkGray);

pub const BORDER: Style = Style::new()
    .fg(Color::DarkGray);

pub const TITLE: Style = Style::new()
    .fg(Color::White);

pub const HINT: Style = Style::new()
    .fg(Color::DarkGray)
    .add_modifier(Modifier::DIM);
