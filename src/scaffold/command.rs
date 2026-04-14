use std::path::PathBuf;
use std::process::Command;

pub fn run_in(dir: &PathBuf, program: &str, args: &[&str]) -> Result<(), String> {
    let status = Command::new(program)
        .args(args)
        .current_dir(dir)
        .status()
        .map_err(|e| format!("Failed to run `{program}`: {e}"))?;

    if status.success() {
        Ok(())
    } else {
        Err(format!(
            "`{program} {}` failed with exit code {:?}",
            args.join(" "),
            status.code()
        ))
    }
}
