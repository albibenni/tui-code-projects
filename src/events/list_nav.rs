use ratatui::widgets::ListState;

#[derive(Clone, Copy)]
pub enum Direction {
    Up,
    Down,
}

pub fn move_list_selection(state: &mut ListState, count: usize, direction: Direction) {
    if count == 0 {
        state.select(None);
        return;
    }

    let current = state.selected().unwrap_or(0).min(count - 1);
    let next = match direction {
        Direction::Up => current.saturating_sub(1),
        Direction::Down => (current + 1).min(count - 1),
    };

    state.select(Some(next));
}
