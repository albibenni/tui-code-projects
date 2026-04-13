use super::shared::centered_rect;
use crate::app::App;
use crate::config::ConfigField;
use crate::style::theme;
use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Position};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, BorderType, Paragraph};

pub fn draw(frame: &mut Frame, app: &mut App) {
    let area = centered_rect(52, 14, frame.area());

    let category_label = app.selected_category.map(|c| c.label()).unwrap_or("");
    let lang_name = app.selected_language().map(|l| l.name).unwrap_or("");

    let outer = Block::bordered()
        .border_type(BorderType::Rounded)
        .border_style(theme::BORDER)
        .title_top(Span::styled(
            format!(" new-project — {} — {} ", category_label, lang_name),
            theme::TITLE,
        ))
        .title_bottom(
            Line::from(Span::styled(
                " tab switch  enter confirm  esc back ",
                theme::HINT,
            ))
            .right_aligned(),
        );

    let inner = outer.inner(area);
    frame.render_widget(outer, area);

    let rows = Layout::vertical([
        Constraint::Length(1), // gap
        Constraint::Length(1), // name label
        Constraint::Length(3), // name input
        Constraint::Length(1), // gap
        Constraint::Length(1), // path label
        Constraint::Length(3), // path input
        Constraint::Fill(1),   // error / gap
    ])
    .split(inner);

    // Name label
    frame.render_widget(
        Paragraph::new(Span::styled(" Project Name", theme::UNSELECTED)),
        rows[1],
    );

    // Name input
    let name_active = app.config.active_field == ConfigField::Name;
    let name_block = Block::bordered()
        .border_type(BorderType::Rounded)
        .border_style(if name_active { theme::SELECTED } else { theme::BORDER });
    let name_inner = name_block.inner(rows[2]);
    frame.render_widget(name_block, rows[2]);
    frame.render_widget(
        Paragraph::new(app.config.project_name.as_str()),
        name_inner,
    );

    // Path label
    frame.render_widget(
        Paragraph::new(Span::styled(" Project Path", theme::UNSELECTED)),
        rows[4],
    );

    // Path input
    let path_active = app.config.active_field == ConfigField::Path;
    let path_block = Block::bordered()
        .border_type(BorderType::Rounded)
        .border_style(if path_active { theme::SELECTED } else { theme::BORDER });
    let path_inner = path_block.inner(rows[5]);
    frame.render_widget(path_block, rows[5]);
    frame.render_widget(
        Paragraph::new(app.config.project_path.as_str()),
        path_inner,
    );

    // Error message
    if let Some(ref err) = app.config.error_message {
        frame.render_widget(
            Paragraph::new(Span::styled(format!(" {}", err), theme::ERROR)),
            rows[6],
        );
    }

    // Terminal cursor at end of active input
    let (cursor_area, value_len) = if name_active {
        (name_inner, app.config.project_name.len())
    } else {
        (path_inner, app.config.project_path.len())
    };
    frame.set_cursor_position(Position {
        x: cursor_area.x + value_len as u16,
        y: cursor_area.y,
    });
}
