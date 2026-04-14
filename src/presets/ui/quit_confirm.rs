use super::shared::centered_rect;
use crate::style::theme;
use ratatui::Frame;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, BorderType, Clear, Paragraph};

pub fn draw(frame: &mut Frame) {
    let area = centered_rect(32, 5, frame.area());

    frame.render_widget(Clear, area);

    let block = Block::bordered()
        .border_type(BorderType::Rounded)
        .border_style(theme::SELECTED)
        .title_top(Span::styled(" quit? ", theme::TITLE))
        .title_bottom(
            Line::from(Span::styled(" y yes  n no ", theme::HINT)).right_aligned(),
        );

    let text = Line::from(vec![
        Span::styled("  Are you sure you want to quit?", theme::UNSELECTED),
    ]);

    frame.render_widget(Paragraph::new(vec![Line::from(""), text]).block(block), area);
}
