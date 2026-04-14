use std::path::PathBuf;
use std::sync::mpsc::Sender;

use super::command::run_in;
use super::params::ScaffoldParams;

pub fn scaffold(params: &ScaffoldParams, base: &PathBuf, tx: &Sender<String>) -> Result<(), String> {
    let _ = tx.send(format!("Running go mod init {}...", params.project_name));
    run_in(base, "go", &["mod", "init", &params.project_name], tx)
}
