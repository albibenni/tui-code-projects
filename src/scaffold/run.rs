use std::fs;
use std::path::PathBuf;

use crate::app::App;

use super::{go, python, rust, typescript_backend, typescript_frontend};

pub fn run(app: &App) -> Result<String, String> {
    let base: PathBuf = [&app.config.project_path, &app.config.project_name]
        .iter()
        .collect();

    fs::create_dir_all(&base).map_err(|e| format!("Failed to create directory: {e}"))?;

    let lang = app.selected_language().map(|l| l.name).unwrap_or("");

    match lang {
        "TypeScript (Backend)"  => typescript_backend::scaffold(app, &base)?,
        "TypeScript (Frontend)" => typescript_frontend::scaffold(app, &base)?,
        "Go"                    => go::scaffold(app, &base)?,
        "Rust"                  => rust::scaffold(app, &base)?,
        "Python"                => python::scaffold(app, &base)?,
        _                       => {}
    }

    Ok(base.display().to_string())
}
