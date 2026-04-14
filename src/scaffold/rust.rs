use std::path::PathBuf;

use crate::app::App;

use super::command::run_in;

pub fn scaffold(app: &App, base: &PathBuf) -> Result<(), String> {
    let project_type = app
        .option_selections
        .iter()
        .find(|s| s.title == "Project Type")
        .map(|s| s.choice_name)
        .unwrap_or("Binary");

    if project_type == "Library" {
        run_in(base, "cargo", &["init", "--lib"])
    } else {
        run_in(base, "cargo", &["init"])
    }
}
