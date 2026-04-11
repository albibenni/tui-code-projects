use crate::app::{App, Step};
use crate::presets::Category;
use crate::style::theme;
use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::text::Span;
use ratatui::widgets::{Block, BorderType, List, ListItem};

pub fn draw(frame: &mut Frame, app: &mut App) {
    match app.step {
        Step::Category => draw_category(frame, app),
        Step::Language => draw_language(frame, app),
        Step::Preset => draw_preset(frame, app),
        Step::Config => draw_config(frame, app),
        Step::Confirm => draw_confirm(frame, app),
        Step::Done => draw_done(frame, app),
    }
}

fn centered_rect(width: u16, height: u16, area: Rect) -> Rect {
    let vertical = Layout::vertical([
        Constraint::Fill(1),
        Constraint::Length(height),
        Constraint::Fill(1),
    ])
    .split(area);

    Layout::horizontal([
        Constraint::Fill(1),
        Constraint::Length(width),
        Constraint::Fill(1),
    ])
    .split(vertical[1])[1]
}

fn draw_category(frame: &mut Frame, app: &mut App) {
    let area = centered_rect(40, 8, frame.area());

    let block = Block::bordered()
        .border_type(BorderType::Rounded)
        .border_style(theme::BORDER)
        .title_top(Span::styled(" new-project ", theme::TITLE))
        .title_bottom(
            ratatui::text::Line::from(Span::styled(
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

fn draw_language(frame: &mut Frame, app: &mut App) {
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
            ratatui::text::Line::from(Span::styled(
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

fn draw_preset(_frame: &mut Frame, _app: &mut App) {
    todo!()
}

fn draw_config(_frame: &mut Frame, _app: &mut App) {
    todo!()
}

fn draw_confirm(_frame: &mut Frame, _app: &mut App) {
    todo!()
}

fn draw_done(_frame: &mut Frame, _app: &mut App) {
    todo!()
}
