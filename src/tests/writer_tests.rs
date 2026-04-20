use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::scaffold::writer::ensure_js_linting_scripts;

fn unique_temp_dir(prefix: &str) -> std::path::PathBuf {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time after UNIX_EPOCH")
        .as_nanos();
    std::env::temp_dir().join(format!("{prefix}-{nonce}"))
}

#[test]
fn ensure_js_linting_scripts_adds_lint_and_prettier_for_prettier_profile() {
    let dir = unique_temp_dir("new-project-tui-writer-test");
    fs::create_dir_all(&dir).expect("create temp dir");
    let package_path = dir.join("package.json");
    fs::write(
        &package_path,
        r#"{
  "name": "app",
  "scripts": {
    "dev": "vite",
    "build": "tsc"
  }
}
"#,
    )
    .expect("write package.json");

    ensure_js_linting_scripts(&dir, "Recommended + Prettier").expect("script update should succeed");
    let updated = fs::read_to_string(&package_path).expect("read updated package.json");

    assert!(updated.contains("\"lint\": \"eslint .\""));
    assert!(updated.contains("\"format\": \"prettier . --write\""));
    assert!(updated.contains("\"format:check\": \"prettier . --check\""));

    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn ensure_js_linting_scripts_does_not_override_existing_lint_script() {
    let dir = unique_temp_dir("new-project-tui-writer-test");
    fs::create_dir_all(&dir).expect("create temp dir");
    let package_path = dir.join("package.json");
    fs::write(
        &package_path,
        r#"{
  "name": "app",
  "scripts": {
    "lint": "next lint",
    "dev": "next dev"
  }
}
"#,
    )
    .expect("write package.json");

    ensure_js_linting_scripts(&dir, "Custom Strict").expect("script update should succeed");
    let updated = fs::read_to_string(&package_path).expect("read updated package.json");

    assert!(updated.contains("\"lint\": \"next lint\""));
    assert!(updated.contains("\"format\": \"prettier . --write\""));
    assert!(updated.contains("\"format:check\": \"prettier . --check\""));

    let _ = fs::remove_dir_all(&dir);
}

