use std::path::PathBuf;

use crate::app::App;

use super::command::run_in;
use super::writer::write_file;

pub fn scaffold(app: &App, base: &PathBuf) -> Result<(), String> {
    let pm = app
        .option_selections
        .iter()
        .find(|s| s.title == "Package Manager")
        .map(|s| s.choice_name)
        .unwrap_or("pip");

    match pm {
        "uv"     => run_in(base, "uv", &["init", "."]),
        "poetry" => run_in(base, "poetry", &["init", "--no-interaction"]),
        "conda"  => write_file(base, "environment.yml", &conda_env(app)),
        _        => write_file(base, "requirements.txt", "# Add your dependencies here\n"),
    }
}

fn conda_env(app: &App) -> String {
    format!(
        "name: {}\nchannels:\n  - defaults\ndependencies:\n  - python>=3.11\n",
        app.config.project_name
    )
}
