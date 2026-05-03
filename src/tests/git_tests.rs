use std::fs;
use std::process::Command;
use std::sync::mpsc;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::scaffold::{ScaffoldParams, run_threaded};

fn run(params: ScaffoldParams) -> Vec<String> {
    let (tx, rx) = mpsc::channel();
    run_threaded(params, tx);
    rx.iter().collect()
}

fn create_params(
    path: String,
    name: &str,
    lang: &str,
    selections: Vec<(&str, &str)>,
) -> ScaffoldParams {
    ScaffoldParams {
        project_path: path,
        project_name: name.to_string(),
        language_name: lang.to_string(),
        selections: selections
            .into_iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect(),
    }
}

#[test]
fn run_threaded_inits_git_even_inside_another_repo() {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time after UNIX_EPOCH")
        .as_nanos();
    let root = std::env::temp_dir().join(format!("new-project-tui-parent-git-{}", nonce));
    fs::create_dir_all(&root).expect("create test root");

    // Initialize parent git repo
    Command::new("git")
        .arg("init")
        .arg("-q")
        .current_dir(&root)
        .status()
        .expect("parent git init failed");

    let project_name = "rust-nested-app";
    let params = create_params(
        root.display().to_string(),
        project_name,
        "Rust",
        vec![("Project Type", "Binary")],
    );

    let _lines = run(params);

    let git_dir = root.join(project_name).join(".git");
    assert!(
        git_dir.exists(),
        "Git repository should be initialized even if inside another repo"
    );

    let _ = fs::remove_dir_all(root.join(project_name));
    let _ = fs::remove_dir_all(root);
}

#[test]
fn run_threaded_inits_git_for_swift() {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time after UNIX_EPOCH")
        .as_nanos();
    let root = std::env::temp_dir().join(format!("new-project-tui-git-swift-{}", nonce));
    fs::create_dir_all(&root).expect("create test root");

    let project_name = "swift-app";
    let params = create_params(
        root.display().to_string(),
        project_name,
        "Swift",
        vec![("UI Framework", "SwiftUI"), ("Core Language", "Swift Only")],
    );

    let _lines = run(params);

    let git_dir = root.join(project_name).join(".git");
    assert!(
        git_dir.exists(),
        "Git repository should be initialized for Swift projects"
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
    let root = std::env::temp_dir().join(format!("new-project-tui-git-ts-{}", nonce));
    fs::create_dir_all(&root).expect("create test root");

    let project_name = "ts-app";
    let params = create_params(
        root.display().to_string(),
        project_name,
        "TypeScript (Backend)",
        vec![
            ("Runtime", "Node"),
            ("Framework", "Express"),
            ("Package Manager", "npm"),
        ],
    );

    let _lines = run(params);

    let git_dir = root.join(project_name).join(".git");
    assert!(
        git_dir.exists(),
        "Git repository should be initialized for TS projects"
    );

    let _ = fs::remove_dir_all(root.join(project_name));
    let _ = fs::remove_dir_all(root);
}

#[test]
fn run_threaded_inits_git_for_rust() {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time after UNIX_EPOCH")
        .as_nanos();
    let root = std::env::temp_dir().join(format!("new-project-tui-git-rust-{}", nonce));
    fs::create_dir_all(&root).expect("create test root");

    let project_name = "rust-app";
    let params = create_params(
        root.display().to_string(),
        project_name,
        "Rust",
        vec![("Project Type", "Binary")],
    );

    let _lines = run(params);

    let git_dir = root.join(project_name).join(".git");
    assert!(
        git_dir.exists(),
        "Git repository should be initialized for Rust projects"
    );

    let _ = fs::remove_dir_all(root.join(project_name));
    let _ = fs::remove_dir_all(root);
}

#[test]
fn run_threaded_inits_git_for_rust_desktop() {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time after UNIX_EPOCH")
        .as_nanos();
    let root = std::env::temp_dir().join(format!("new-project-tui-git-rust-desk-{}", nonce));
    fs::create_dir_all(&root).expect("create test root");

    let project_name = "rust-desk-app";
    let params = create_params(
        root.display().to_string(),
        project_name,
        "Rust (Desktop)",
        vec![("Framework", "Tauri")],
    );

    let _lines = run(params);

    let git_dir = root.join(project_name).join(".git");
    assert!(
        git_dir.exists(),
        "Git repository should be initialized for Rust (Desktop) projects"
    );

    let _ = fs::remove_dir_all(root.join(project_name));
    let _ = fs::remove_dir_all(root);
}

#[test]
fn run_threaded_inits_git_for_python() {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time after UNIX_EPOCH")
        .as_nanos();
    let root = std::env::temp_dir().join(format!("new-project-tui-git-py-{}", nonce));
    fs::create_dir_all(&root).expect("create test root");

    let project_name = "py-app";
    let params = create_params(
        root.display().to_string(),
        project_name,
        "Python",
        vec![("Project Type", "Script"), ("Package Manager", "pip")],
    );

    let _lines = run(params);

    let git_dir = root.join(project_name).join(".git");
    assert!(
        git_dir.exists(),
        "Git repository should be initialized for Python projects"
    );

    let _ = fs::remove_dir_all(root.join(project_name));
    let _ = fs::remove_dir_all(root);
}

#[test]
fn ensure_git_repo_skips_if_already_exists() {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let root = std::env::temp_dir().join(format!("git-exists-test-{}", nonce));
    fs::create_dir_all(&root).unwrap();

    // Pre-create .git directory
    fs::create_dir_all(root.join(".git")).unwrap();

    let (tx, rx) = mpsc::channel();
    // We call it directly to test its logic
    crate::scaffold::ensure_git_repo(&root, &tx);

    // It should NOT send "Initializing git repository..." because it skipped
    let messages: Vec<String> = rx.try_iter().collect();
    assert!(
        !messages
            .iter()
            .any(|m| m.contains("Initializing git repository"))
    );

    let _ = fs::remove_dir_all(root);
}

#[test]
fn run_threaded_inits_git_for_go() {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time after UNIX_EPOCH")
        .as_nanos();
    let root = std::env::temp_dir().join(format!("new-project-tui-git-go-{}", nonce));
    fs::create_dir_all(&root).expect("create test root");

    let project_name = "go-app";
    let params = create_params(root.display().to_string(), project_name, "Go", vec![]);

    let _lines = run(params);

    let git_dir = root.join(project_name).join(".git");
    assert!(
        git_dir.exists(),
        "Git repository should be initialized for Go projects"
    );

    let _ = fs::remove_dir_all(root.join(project_name));
    let _ = fs::remove_dir_all(root);
}
