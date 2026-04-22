use super::shared::centered_rect;
use crate::app::App;
use crate::style::theme;
use ratatui::Frame;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, BorderType, Paragraph, Wrap};

pub fn draw(frame: &mut Frame, app: &App) {
    let area = centered_rect(72, 24, frame.area());
    let inner_width = area.width.saturating_sub(4) as usize;
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

    let mut lines = Vec::new();
    for l in &app.output_lines {
        // Simple ANSI escape code stripper
        let mut clean = String::new();
        let mut chars = l.chars();
        while let Some(c) = chars.next() {
            if c == '\x1B' {
                // Skip until we find the end of the ANSI sequence (a letter)
                while let Some(next) = chars.next() {
                    if next.is_ascii_alphabetic() {
                        break;
                    }
                }
            } else if !c.is_control() || c == '\n' {
                clean.push(c);
            }
        }

        for part in clean.split('\n') {
            if part.len() <= inner_width {
                lines.push(part.to_string());
            } else {
                let mut current = part;
                while current.len() > inner_width {
                    let (left, right) = current.split_at(inner_width);
                    lines.push(left.to_string());
                    current = right;
                }
                lines.push(current.to_string());
            }
        }
    }

    let visible_lines: Vec<Line> = lines
        .iter()
        .rev()
        .take(inner_height)
        .rev()
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
