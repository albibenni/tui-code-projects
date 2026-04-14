use std::path::PathBuf;

use crate::app::App;

use super::command::run_in;

pub fn scaffold(app: &App, base: &PathBuf) -> Result<(), String> {
    let module = &app.config.project_name;
    run_in(base, "go", &["mod", "init", module])
}
