use super::shared::centered_rect;
use crate::app::App;
use crate::style::theme;
use ratatui::Frame;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, BorderType, List, ListItem};

pub fn draw(frame: &mut Frame, app: &mut App) {
    let languages = app.filtered_languages();
    let count = languages.len();
    let height = (count as u16 + 4).max(6);
    let area = centered_rect(44, height, frame.area());

    let category_label = app
        .selected_category
        .map(|c| c.label())
        .unwrap_or("Languages");

    let block = Block::bordered()
        .border_type(BorderType::Rounded)
        .border_style(theme::BORDER)
        .title_top(Span::styled(
            format!(" new-project — {} ", category_label),
            theme::TITLE,
        ))
        .title_bottom(
            Line::from(Span::styled(
                " ↑↓ navigate  enter select  b back  q quit ",
                theme::HINT,
            ))
            .right_aligned(),
        );

    let items: Vec<ListItem> = languages
        .iter()
        .map(|l| ListItem::new(format!(" {} ", l.name)))
        .collect();

    let list = List::new(items)
        .block(block)
        .highlight_style(theme::SELECTED)
        .highlight_symbol("> ");

    frame.render_stateful_widget(list, area, &mut app.lang_state);
}
