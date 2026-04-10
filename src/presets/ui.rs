use crate::app::{App, Step};
use ratatui::Frame;

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

fn draw_category(_frame: &mut Frame, _app: &mut App) {
    todo!()
}

fn draw_language(_frame: &mut Frame, _app: &mut App) {
    todo!()
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
