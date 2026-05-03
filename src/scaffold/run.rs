use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::sync::mpsc::Sender;

use crate::config::validate_project_name;

use super::params::ScaffoldParams;
use super::{
    flutter, go, go_desktop, java, kotlin_mobile, php, python, python_desktop, rust, rust_desktop,
    swift_desktop, swift_mobile, typescript_backend, typescript_desktop, typescript_frontend,
};

use std::sync::atomic::Ordering;

pub fn run_threaded(params: ScaffoldParams, tx: Sender<String>) {
    if let Err(e) = validate_project_name(&params.project_name, Some(&params.language_name)) {
        let _ = tx.send(format!("Error: {e}"));
        return;
    }

    let base: PathBuf = [&params.project_path, &params.project_name]
        .iter()
        .collect();

    if let Err(e) = execute(&params, &base, &tx) {
        let _ = tx.send(format!("Error: {e}"));
        
        if !is_dangerous(&base) && base.exists() && base.is_dir() {
            let _ = tx.send("Cleaning up...".to_string());
            if let Err(cleanup_err) = fs::remove_dir_all(&base) {
                let _ = tx.send(format!("Warning: cleanup failed: {cleanup_err}"));
            }
        }
    }
}

pub fn cleanup(project_path: &str, project_name: &str) {
    let base: PathBuf = [project_path, project_name].iter().collect();

    if !is_dangerous(&base) && base.exists() && base.is_dir() {
        if let Err(e) = fs::remove_dir_all(&base) {
            eprintln!("\nWarning: cleanup failed for {}: {}", base.display(), e);
        }
    }
}

fn is_dangerous(path: &Path) -> bool {
    path.as_os_str().is_empty()
        || path == Path::new(".")
        || path == Path::new("..")
        || (path.is_relative()
            && (path.starts_with(".") || path.starts_with(".."))
            && path.components().count() < 2)
}

fn execute(params: &ScaffoldParams, base: &PathBuf, tx: &Sender<String>) -> Result<(), String> {
    if super::INTERRUPTED.load(Ordering::SeqCst) {
        return Err("Interrupted".to_string());
    }

    fs::create_dir_all(base).map_err(|e| format!("Failed to create directory: {e}"))?;

    match params.language_name.as_str() {
        "TypeScript (Backend)" => typescript_backend::scaffold(params, &base, tx)?,
        "TypeScript (Frontend)" => typescript_frontend::scaffold(params, &base, tx)?,
        "TypeScript (Desktop)" => typescript_desktop::scaffold(params, &base, tx)?,
        "Flutter" => flutter::scaffold(params, &base, tx)?,
        "Kotlin (Mobile)" => kotlin_mobile::scaffold(params, &base, tx)?,
        "Swift (Mobile)" => swift_mobile::scaffold(params, &base, tx)?,
        "Go" => go::scaffold(params, &base, tx)?,
        "Go (Desktop)" => go_desktop::scaffold(params, &base, tx)?,
        "Java" => java::scaffold(params, &base, tx)?,
        "PHP" => php::scaffold(params, &base, tx)?,
        "Python (Desktop)" => python_desktop::scaffold(params, &base, tx)?,
        "Rust" => rust::scaffold(params, &base, tx)?,
        "Rust (Desktop)" => rust_desktop::scaffold(params, &base, tx)?,
        "Swift" => swift_desktop::scaffold(params, &base, tx)?,
        "Python" => python::scaffold(params, &base, tx)?,
        _ => {}
    }

    if super::INTERRUPTED.load(Ordering::SeqCst) {
        return Err("Interrupted".to_string());
    }

    ensure_git_repo(&base, tx);
    apply_git_hooks(&base, params, tx);

    let _ = tx.send(format!("Done — project created at {}", base.display()));
    Ok(())
}

pub fn ensure_git_repo(base: &Path, tx: &Sender<String>) {
    if base.join(".git").exists() {
        return;
    }

    if !super::command::command_exists("git") {
        let _ = tx.send("Warning: `git` command not found in PATH. Skipping git initialization.".to_string());
        return;
    }

    let _ = tx.send("Initializing git repository...".to_string());
    if let Err(e) = super::command::run_in(base, "git", &["init", "-q"], tx) {
        let _ = tx.send(format!("Warning: git initialization failed: {e}"));
    } else {
        let _ = tx.send("Git repository initialized successfully.".to_string());
    }
}

fn apply_git_hooks(base: &Path, params: &ScaffoldParams, tx: &Sender<String>) {
    let hook_choice = params.sel("Git Hooks").unwrap_or("None");
    if hook_choice == "None" {
        return;
    }

    if !base.join(".git").exists() {
        let _ = tx.send("Warning: skipping git hook setup because .git was not found.".to_string());
        return;
    }

    match hook_choice {
        "Husky (lint + test)" => {
            if let Err(e) = setup_husky_hook(base, params, tx) {
                let _ = tx.send(format!("Warning: failed to setup Husky hook: {e}"));
            }
        }
        "Native Git Hook (make lint && make test)" => {
            if let Err(e) = setup_native_hook(base) {
                let _ = tx.send(format!("Warning: failed to setup native git hook: {e}"));
            }
        }
        "Lefthook (lint + test)" => {
            if let Err(e) = setup_lefthook(base) {
                let _ = tx.send(format!("Warning: failed to setup Lefthook: {e}"));
            }
        }
        _ => {}
    }
}

fn setup_husky_hook(base: &Path, params: &ScaffoldParams, tx: &Sender<String>) -> Result<(), String> {
    let husky_dir = base.join(".husky");
    fs::create_dir_all(&husky_dir).map_err(|e| format!("Failed to create .husky/: {e}"))?;

    let pm = params.sel("Package Manager").unwrap_or("npm");
    let lint_cmd = js_script_run_cmd(pm, "lint");
    let test_cmd = js_script_run_cmd(pm, "test");
    let script = format!(
        "#!/usr/bin/env sh
set -e

if [ ! -f package.json ]; then
  echo \"Skipping Husky pre-commit: package.json not found.\"
  exit 0
fi

if grep -q '\"lint\"' package.json; then
  {lint_cmd}
fi

if grep -q '\"test\"' package.json; then
  {test_cmd}
fi
"
    );

    write_executable(&husky_dir.join("pre-commit"), &script)?;

    super::command::run_in(base, "git", &["config", "core.hooksPath", ".husky"], tx)
}

fn setup_native_hook(base: &Path) -> Result<(), String> {
    let script = r#"#!/usr/bin/env sh
set -e

if [ -f Makefile ]; then
  make lint
  make test
elif [ -x ./gradlew ]; then
  ./gradlew lint test
else
  echo "No lint/test command configured (expected Makefile or ./gradlew)."
  exit 1
fi
"#;

    write_executable(&base.join(".git/hooks/pre-commit"), script)
}

fn setup_lefthook(base: &Path) -> Result<(), String> {
    let config = r#"pre-commit:
  parallel: false
  commands:
    lint:
      run: make lint
    test:
      run: make test
"#;

    fs::write(base.join("lefthook.yml"), config)
        .map_err(|e| format!("Failed to write lefthook.yml: {e}"))?;

    let script = r#"#!/usr/bin/env sh
set -e

command -v lefthook >/dev/null 2>&1 || {
  echo "lefthook is required. Install it and re-run commit."
  exit 1
}

lefthook run pre-commit
"#;

    write_executable(&base.join(".git/hooks/pre-commit"), script)
}

fn js_script_run_cmd(pm: &str, script: &str) -> String {
    match pm {
        "pnpm" => format!("pnpm {script}"),
        "yarn" => format!("yarn {script}"),
        "bun" => format!("bun run {script}"),
        _ => format!("npm run {script}"),
    }
}

fn write_executable(path: &Path, content: &str) -> Result<(), String> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .map_err(|e| format!("Failed to create {}: {e}", parent.display()))?;
    }

    fs::write(path, content).map_err(|e| format!("Failed to write {}: {e}", path.display()))?;

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(path)
            .map_err(|e| format!("Failed to stat {}: {e}", path.display()))?
            .permissions();
        perms.set_mode(0o755);
        fs::set_permissions(path, perms)
            .map_err(|e| format!("Failed to chmod {}: {e}", path.display()))?;
    }

    Ok(())
}
