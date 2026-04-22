use std::io::{BufRead, BufReader};
use std::path::Path;
use std::process::{Command, Stdio};
use std::sync::mpsc::Sender;
use std::thread;

pub fn run_in(dir: &Path, program: &str, args: &[&str], tx: &Sender<String>) -> Result<(), String> {
    let mut child = Command::new(program)
        .args(args)
        .current_dir(dir)
        .stdin(Stdio::null())
        .env("CI", "true")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| format!("Failed to start `{program}`: {e}"))?;

    let stdout = child.stdout.take().ok_or("Failed to capture stdout")?;
    let stderr = child.stderr.take().ok_or("Failed to capture stderr")?;

    let tx_out = tx.clone();
    let tx_err = tx.clone();

    let t_out = thread::spawn(move || {
        for line in BufReader::new(stdout).lines().map_while(Result::ok) {
            let _ = tx_out.send(line);
        }
    });
    let t_err = thread::spawn(move || {
        for line in BufReader::new(stderr).lines().map_while(Result::ok) {
            let _ = tx_err.send(line);
        }
    });

    let status = loop {
        if super::INTERRUPTED.load(std::sync::atomic::Ordering::SeqCst) {
            let _ = child.kill();
            return Err("Interrupted".to_string());
        }

        if let Some(status) = child.try_wait().map_err(|e| format!("Wait failed: {e}"))? {
            break status;
        }

        thread::sleep(std::time::Duration::from_millis(50));
    };

    t_out.join().ok();
    t_err.join().ok();

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

pub fn command_exists(cmd: &str) -> bool {
    Command::new("which")
        .arg(cmd)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}
