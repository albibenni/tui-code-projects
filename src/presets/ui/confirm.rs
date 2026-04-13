use super::shared::centered_rect;
use crate::app::App;
use crate::style::theme;
use ratatui::Frame;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, BorderType, Paragraph};

pub fn draw(frame: &mut Frame, app: &App) {
    let category_label = app.selected_category.map(|c| c.label()).unwrap_or("—");
    let lang_name = app.selected_language().map(|l| l.name).unwrap_or("—");

    let mut lines: Vec<Line> = Vec::new();

    lines.push(Line::from(""));
    lines.push(Line::from(vec![
        Span::styled("  Category    ", theme::HINT),
        Span::styled(category_label, theme::SELECTED),
    ]));
    lines.push(Line::from(vec![
        Span::styled("  Language    ", theme::HINT),
        Span::styled(lang_name, theme::SELECTED),
    ]));

    if !app.option_selections.is_empty() {
        lines.push(Line::from(""));
        for sel in &app.option_selections {
            lines.push(Line::from(vec![
                Span::styled(format!("  {:<14}", sel.title), theme::HINT),
                Span::styled(sel.choice_name, theme::SELECTED),
            ]));
        }
    }

    lines.push(Line::from(""));
    lines.push(Line::from(vec![
        Span::styled("  Name        ", theme::HINT),
        Span::styled(app.config.project_name.as_str(), theme::SELECTED),
    ]));
    lines.push(Line::from(vec![
        Span::styled("  Path        ", theme::HINT),
        Span::styled(app.config.project_path.as_str(), theme::SELECTED),
    ]));
    lines.push(Line::from(""));

    let height = lines.len() as u16 + 4;
    let area = centered_rect(56, height, frame.area());

    let block = Block::bordered()
        .border_type(BorderType::Rounded)
        .border_style(theme::BORDER)
        .title_top(Span::styled(" new-project — confirm ", theme::TITLE))
        .title_bottom(
            Line::from(Span::styled(
                " enter confirm  b back  q quit ",
                theme::HINT,
            ))
            .right_aligned(),
        );

    frame.render_widget(Paragraph::new(lines).block(block), area);
}
