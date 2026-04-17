use std::fs;
use std::path::Path;
use std::sync::mpsc::Sender;

use super::command::run_in;
use super::params::ScaffoldParams;
use super::writer::write_file;

pub fn scaffold(params: &ScaffoldParams, base: &Path, tx: &Sender<String>) -> Result<(), String> {
    let start_config = params
        .sel("Start Configuration")
        .unwrap_or("Mobile (Android + iOS)");
    let state_management = params.sel("State Management").unwrap_or("Provider");

    send(tx, "Running flutter create...");
    let platforms = platforms_for(start_config);
    run_in(
        base,
        "flutter",
        &["create", ".", "--platforms", platforms],
        tx,
    )?;

    add_state_management_package(base, state_management, tx)?;
    write_vscode_launch(base, start_config, tx)?;
    write_file(base, "Makefile", makefile())?;

    Ok(())
}

pub(crate) fn platforms_for(start_config: &str) -> &'static str {
    match start_config {
        "Web" => "web",
        "Desktop" => "linux,macos,windows",
        "All Platforms" => "android,ios,web,linux,macos,windows",
        _ => "android,ios",
    }
}

fn add_state_management_package(
    base: &Path,
    state_management: &str,
    tx: &Sender<String>,
) -> Result<(), String> {
    let package = match state_management {
        "Riverpod" => Some("flutter_riverpod"),
        "BLoC" => Some("flutter_bloc"),
        "Provider" => Some("provider"),
        _ => None,
    };

    if let Some(package) = package {
        send(tx, format!("Adding state management package: {package}..."));
        run_in(base, "flutter", &["pub", "add", package], tx)?;
    } else {
        send(tx, "Skipping state management dependency install.");
    }

    Ok(())
}

fn write_vscode_launch(base: &Path, start_config: &str, tx: &Sender<String>) -> Result<(), String> {
    send(tx, "Writing VS Code launch config...");
    let vscode_dir = base.join(".vscode");
    fs::create_dir_all(&vscode_dir)
        .map_err(|e| format!("Failed to create .vscode directory: {e}"))?;

    let launch_json = launch_json_for(start_config);
    fs::write(vscode_dir.join("launch.json"), launch_json)
        .map_err(|e| format!("Failed to write launch.json: {e}"))?;
    Ok(())
}

pub(crate) fn launch_json_for(start_config: &str) -> String {
    let mut configs = vec![
        flutter_config("Flutter Debug (Auto Device)", None),
        flutter_config("Flutter Profile (Auto Device)", Some("profile")),
        flutter_config("Flutter Release (Auto Device)", Some("release")),
    ];

    if matches!(start_config, "Web" | "All Platforms") {
        configs.push(web_config("Flutter Web (Chrome)", None));
        configs.push(web_config("Flutter Web (Chrome, Profile)", Some("profile")));
    }

    let joined = configs.join(",\n");
    format!("{{\n  \"version\": \"0.2.0\",\n  \"configurations\": [\n{joined}\n  ]\n}}\n")
}

fn flutter_config(name: &str, mode: Option<&str>) -> String {
    let mode_line = mode
        .map(|m| format!(",\n      \"flutterMode\": \"{m}\""))
        .unwrap_or_default();
    format!(
        "    {{\n      \"name\": \"{name}\",\n      \"request\": \"launch\",\n      \"type\": \"dart\"{mode_line}\n    }}"
    )
}

fn web_config(name: &str, mode: Option<&str>) -> String {
    let mode_line = mode
        .map(|m| format!(",\n      \"flutterMode\": \"{m}\""))
        .unwrap_or_default();
    format!(
        "    {{\n      \"name\": \"{name}\",\n      \"request\": \"launch\",\n      \"type\": \"dart\",\n      \"deviceId\": \"chrome\"{mode_line}\n    }}"
    )
}

fn send(tx: &Sender<String>, msg: impl Into<String>) {
    let _ = tx.send(msg.into());
}

fn makefile() -> &'static str {
    r#"FLUTTER ?= flutter

.PHONY: setup run test analyze clean

setup:
	@$(FLUTTER) pub get

run:
	@$(FLUTTER) run

test:
	@$(FLUTTER) test

analyze:
	@$(FLUTTER) analyze

clean:
	@$(FLUTTER) clean
"#
}
