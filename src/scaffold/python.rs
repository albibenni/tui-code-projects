use std::path::PathBuf;
use std::sync::mpsc::Sender;

use super::command::run_in;
use super::params::ScaffoldParams;
use super::writer::write_file;

pub fn scaffold(params: &ScaffoldParams, base: &PathBuf, tx: &Sender<String>) -> Result<(), String> {
    let pm = params.sel("Package Manager").unwrap_or("pip");

    match pm {
        "uv" => {
            let _ = tx.send("Running uv init...".to_string());
            run_in(base, "uv", &["init", "."], tx)
        }
        "poetry" => {
            let _ = tx.send("Running poetry init...".to_string());
            run_in(base, "poetry", &["init", "--no-interaction"], tx)
        }
        "conda" => {
            let _ = tx.send("Writing environment.yml...".to_string());
            write_file(base, "environment.yml", &conda_env(params))
        }
        _ => {
            let _ = tx.send("Writing requirements.txt...".to_string());
            write_file(base, "requirements.txt", "# Add your dependencies here\n")
        }
    }
}

fn conda_env(params: &ScaffoldParams) -> String {
    format!(
        "name: {}\nchannels:\n  - defaults\ndependencies:\n  - python>=3.11\n",
        params.project_name
    )
}
