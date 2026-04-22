use super::shared::centered_rect;
use crate::app::App;
use crate::style::theme;
use ratatui::Frame;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, BorderType, Paragraph, Wrap};

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
            // Strip \r and control characters that might mess up TUI rendering
            let clean_line: String = l.chars().filter(|c| *c != '\r').collect();
            
            let style = if clean_line.starts_with("Error:") {
                theme::ERROR
            } else if clean_line.starts_with("Done") {
                theme::SELECTED
            } else {
                theme::HINT
            };
            Line::from(Span::styled(format!(" {clean_line}"), style))
        })
        .collect();

    frame.render_widget(Paragraph::new(visible_lines).block(block).wrap(Wrap { trim: false }), area);
}
