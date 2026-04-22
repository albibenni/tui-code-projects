use super::shared::centered_rect;
use crate::app::App;
use crate::style::theme;
use ratatui::Frame;
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{Block, BorderType, List, ListItem};

pub fn draw(frame: &mut Frame, app: &mut App) {
    let option_step = match app.current_option_step() {
        Some(s) => s,
        None => return,
    };

    let count = option_step.choices.len();
    let is_multi = option_step.is_multi;
    let height = (count as u16 * 2 + 4).max(8);
    let area = centered_rect(56, height, frame.area());

    let category_label = app.selected_category.map(|c| c.label()).unwrap_or("");
    let lang_name = app.selected_language().map(|l| l.name).unwrap_or("");
    let step_title = option_step.title;

    let hint = if is_multi {
        " ↑↓ navigate  space toggle  enter confirm  b back  q quit "
    } else {
        " ↑↓ navigate  enter select  b back  q quit "
    };

    let block = Block::bordered()
        .border_type(BorderType::Rounded)
        .border_style(theme::BORDER)
        .title_top(Span::styled(
            format!(
                " new-project — {} — {} — {} ",
                category_label, lang_name, step_title
            ),
            theme::TITLE,
        ))
        .title_bottom(
            Line::from(Span::styled(hint, theme::HINT)).right_aligned(),
        );

    let items: Vec<ListItem> = option_step
        .choices
        .iter()
        .enumerate()
        .map(|(i, c)| {
            let prefix = if is_multi {
                if app.current_multi_indices.contains(&i) {
                    "[x] "
                } else {
                    "[ ] "
                }
            } else {
                ""
            };

            ListItem::new(Text::from(vec![
                Line::from(Span::raw(format!(" {}{} ", prefix, c.name))),
                Line::from(Span::styled(
                    format!("   {} ", c.description),
                    theme::UNSELECTED,
                )),
            ]))
        })
        .collect();

    let list = List::new(items)
        .block(block)
        .highlight_style(theme::SELECTED)
        .highlight_symbol("> ");

    frame.render_stateful_widget(list, area, &mut app.option_list_state);
}
