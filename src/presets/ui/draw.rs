use super::{category, config, confirm, done, language, options};
use crate::app::{App, Step};
use ratatui::Frame;

pub fn draw(frame: &mut Frame, app: &mut App) {
    match app.step {
        Step::Category => category::draw(frame, app),
        Step::Language => language::draw(frame, app),
        Step::Options  => options::draw(frame, app),
        Step::Config   => config::draw(frame, app),
        Step::Confirm  => confirm::draw(frame, app),
        Step::Done     => done::draw(frame, app),
    }
}
