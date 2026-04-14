use std::fs;
use std::path::PathBuf;
use std::sync::mpsc::Sender;

use super::params::ScaffoldParams;
use super::{go, python, rust, typescript_backend, typescript_frontend};

pub fn run_threaded(params: ScaffoldParams, tx: Sender<String>) {
    if let Err(e) = execute(&params, &tx) {
        let _ = tx.send(format!("Error: {e}"));
    }
    // tx dropped here — signals completion to the main thread
}

fn execute(params: &ScaffoldParams, tx: &Sender<String>) -> Result<(), String> {
    let base: PathBuf = [&params.project_path, &params.project_name]
        .iter()
        .collect();

    fs::create_dir_all(&base).map_err(|e| format!("Failed to create directory: {e}"))?;

    match params.language_name.as_str() {
        "TypeScript (Backend)"  => typescript_backend::scaffold(params, &base, tx)?,
        "TypeScript (Frontend)" => typescript_frontend::scaffold(params, &base, tx)?,
        "Go"                    => go::scaffold(params, &base, tx)?,
        "Rust"                  => rust::scaffold(params, &base, tx)?,
        "Python"                => python::scaffold(params, &base, tx)?,
        _                       => {}
    }

    let _ = tx.send(format!("Done — project created at {}", base.display()));
    Ok(())
}
