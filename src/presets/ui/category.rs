use super::shared::centered_rect;
use crate::app::App;
use crate::presets::Category;
use crate::style::theme;
use ratatui::Frame;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, BorderType, List, ListItem};

pub fn draw(frame: &mut Frame, app: &mut App) {
    let area = centered_rect(40, 8, frame.area());

    let block = Block::bordered()
        .border_type(BorderType::Rounded)
        .border_style(theme::BORDER)
        .title_top(Span::styled(" new-project ", theme::TITLE))
        .title_bottom(
            Line::from(Span::styled(
                " ↑↓ navigate  enter select  q quit ",
                theme::HINT,
            ))
            .right_aligned(),
        );

    let items: Vec<ListItem> = Category::all()
        .iter()
        .map(|c| ListItem::new(format!(" {} ", c.label())))
        .collect();

    let list = List::new(items)
        .block(block)
        .highlight_style(theme::SELECTED)
        .highlight_symbol("> ");

    frame.render_stateful_widget(list, area, &mut app.category_state);
}
