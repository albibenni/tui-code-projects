use super::shared::centered_rect;
use crate::app::App;
use crate::style::theme;
use ratatui::Frame;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, BorderType, Paragraph};

pub fn draw(frame: &mut Frame, app: &App) {
    let area = centered_rect(72, 24, frame.area());
    let inner_height = area.height.saturating_sub(2) as usize;

    let hint = if app.scaffold_done {
        " enter continue  q quit "
    } else {
        " scaffolding… "
    };

    let block = Block::bordered()
        .border_type(BorderType::Rounded)
        .border_style(theme::BORDER)
        .title_top(Span::styled(" new-project — running ", theme::TITLE))
        .title_bottom(Line::from(Span::styled(hint, theme::HINT)).right_aligned());

    let visible_lines: Vec<Line> = app
        .output_lines
        .iter()
        .skip(app.output_lines.len().saturating_sub(inner_height))
        .map(|l| {
            let style = if l.starts_with("Error:") {
                theme::ERROR
            } else if l.starts_with("Done") {
                theme::SELECTED
            } else {
                theme::HINT
            };
            Line::from(Span::styled(format!(" {l}"), style))
        })
        .collect();

    frame.render_widget(Paragraph::new(visible_lines).block(block), area);
}
