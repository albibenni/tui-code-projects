use std::fs;
use std::sync::mpsc;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::scaffold::{ScaffoldParams, run_threaded};

fn base_params(project_path: String, project_name: &str) -> ScaffoldParams {
    ScaffoldParams {
        project_path,
        project_name: project_name.to_string(),
        language_name: "Unknown".to_string(),
        selections: vec![],
    }
}

fn run(params: ScaffoldParams) -> Vec<String> {
    let (tx, rx) = mpsc::channel();
    run_threaded(params, tx);
    rx.iter().collect()
}

#[test]
fn run_threaded_rejects_parent_traversal_project_name() {
    let lines = run(base_params("./".to_string(), "../escape"));
    assert!(lines.iter().any(|line| line.starts_with("Error:")));
}

#[test]
fn run_threaded_rejects_absolute_project_name() {
    let lines = run(base_params("./".to_string(), "/tmp/escape"));
    assert!(lines.iter().any(|line| line.starts_with("Error:")));
}

#[test]
fn run_threaded_done_path_stays_under_selected_base() {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time after UNIX_EPOCH")
        .as_nanos();
    let root = std::env::temp_dir().join(format!("new-project-tui-{nonce}"));
    fs::create_dir_all(&root).expect("create test root");

    let project_name = "safe-app";
    let params = base_params(root.display().to_string(), project_name);
    let lines = run(params);
    let expected = root.join(project_name).display().to_string();

    assert!(
        lines
            .iter()
            .any(|line| line == &format!("Done — project created at {expected}"))
    );

    let _ = fs::remove_dir_all(root.join(project_name));
    let _ = fs::remove_dir_all(root);
}

#[test]
fn run_threaded_inits_git_for_typescript_backend() {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time after UNIX_EPOCH")
        .as_nanos();
    let root = std::env::temp_dir().join(format!("new-project-tui-git-ts-{nonce}"));
    fs::create_dir_all(&root).expect("create test root");

    let project_name = "ts-app";
    let params = ScaffoldParams {
        project_path: root.display().to_string(),
        project_name: project_name.to_string(),
        language_name: "TypeScript (Backend)".to_string(),
        selections: vec![
            ("Runtime".to_string(), "Node".to_string()),
            ("Framework".to_string(), "Express".to_string()),
            ("Package Manager".to_string(), "npm".to_string()),
        ],
    };
    
    let _lines = run(params);
    
    let git_dir = root.join(project_name).join(".git");
    assert!(git_dir.exists(), "Git repository should be initialized for TS projects");

    let _ = fs::remove_dir_all(root.join(project_name));
    let _ = fs::remove_dir_all(root);
}

#[test]
fn run_threaded_inits_git_for_go() {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time after UNIX_EPOCH")
        .as_nanos();
    let root = std::env::temp_dir().join(format!("new-project-tui-git-{nonce}"));
    fs::create_dir_all(&root).expect("create test root");

    let project_name = "go-app";
    let params = ScaffoldParams {
        project_path: root.display().to_string(),
        project_name: project_name.to_string(),
        language_name: "Go".to_string(),
        selections: vec![],
    };
    
    let _lines = run(params);
    
    let git_dir = root.join(project_name).join(".git");
    assert!(git_dir.exists(), "Git repository should be initialized for Go projects");
    assert!(git_dir.is_dir(), ".git should be a directory");

    let _ = fs::remove_dir_all(root.join(project_name));
    let _ = fs::remove_dir_all(root);
}
