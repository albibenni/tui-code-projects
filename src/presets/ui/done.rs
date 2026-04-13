use super::shared::centered_rect;
use crate::app::App;
use crate::style::theme;
use ratatui::Frame;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, BorderType, Paragraph};

pub fn draw(frame: &mut Frame, app: &App) {
    let is_error = app.result_message.starts_with("Error:");

    let message_style = if is_error { theme::ERROR } else { theme::SELECTED };

    let lines = vec![
        Line::from(""),
        Line::from(Span::styled(&app.result_message, message_style)),
        Line::from(""),
    ];

    let area = centered_rect(64, 7, frame.area());

    let block = Block::bordered()
        .border_type(BorderType::Rounded)
        .border_style(theme::BORDER)
        .title_top(Span::styled(" new-project — done ", theme::TITLE))
        .title_bottom(
            Line::from(Span::styled(" enter/q quit ", theme::HINT)).right_aligned(),
        );

    frame.render_widget(Paragraph::new(lines).centered().block(block), area);
}
