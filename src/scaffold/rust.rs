use std::path::PathBuf;
use std::sync::mpsc::Sender;

use super::command::run_in;
use super::params::ScaffoldParams;

pub fn scaffold(params: &ScaffoldParams, base: &PathBuf, tx: &Sender<String>) -> Result<(), String> {
    let project_type = params.sel("Project Type").unwrap_or("Binary");

    if project_type == "Library" {
        let _ = tx.send("Running cargo init --lib...".to_string());
        run_in(base, "cargo", &["init", "--lib"], tx)
    } else {
        let _ = tx.send("Running cargo init...".to_string());
        run_in(base, "cargo", &["init"], tx)
    }
}
