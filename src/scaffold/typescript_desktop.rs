use std::path::Path;
use std::sync::mpsc::Sender;

use super::command::run_in;
use super::params::ScaffoldParams;
use super::writer;

pub fn scaffold(params: &ScaffoldParams, base: &Path, tx: &Sender<String>) -> Result<(), String> {
    let framework = params.sel("Framework").unwrap_or("Electron");
    let pm = params.sel("Package Manager").unwrap_or("npm");
    let testing = params.sel("Testing").unwrap_or("None");

    match framework {
        "Tauri" => scaffold_tauri(params, base, pm, tx),
        "Electron" => scaffold_electron(params, base, pm, tx),
        "NeutralinoJS" => scaffold_neutralino(base, pm, tx),
        _ => Ok(()),
    }?;

    setup_desktop_testing(base, pm, testing, tx)
}

fn setup_desktop_testing(
    base: &Path,
    pm: &str,
    testing: &str,
    tx: &Sender<String>,
) -> Result<(), String> {
    if testing != "Vitest" {
        return Ok(());
    }

    scaffold_emit(tx, format!("Installing Vitest dependencies ({pm})..."));
    let mut args: Vec<&str> = Vec::new();
    let prog = match pm {
        "pnpm" | "yarn" => {
            args.extend_from_slice(&["add", "-D"]);
            pm
        }
        "bun" => {
            args.extend_from_slice(&["add", "-d"]);
            "bun"
        }
        _ => {
            args.extend_from_slice(&["install", "-D"]);
            "npm"
        }
    };
    args.extend_from_slice(&["vitest", "@vitest/coverage-v8", "happy-dom"]);
    run_in(base, prog, &args, tx)?;

    scaffold_emit(tx, "Writing Vitest config...");
    use super::writer_constants;
    writer::write_file(
        base,
        "vitest.config.ts",
        writer_constants::VITEST_FRONTEND_CONFIG,
    )?;

    let scripts = &[
        ("test", "vitest run"),
        ("test:watch", "vitest"),
        ("test:coverage", "vitest run --coverage"),
    ];
    writer::ensure_package_json_scripts(base, scripts)
}

fn scaffold_tauri(
    _params: &ScaffoldParams,
    base: &Path,
    pm: &str,
    tx: &Sender<String>,
) -> Result<(), String> {
    scaffold_emit(tx, "Running create-tauri-app...");
    let (prog, args): (&str, Vec<&str>) = match pm {
        "pnpm" => (
            "pnpm",
            vec![
                "create",
                "tauri-app",
                ".",
                "--template",
                "vanilla-ts",
                "--yes",
            ],
        ),
        "yarn" => (
            "yarn",
            vec![
                "create",
                "tauri-app",
                ".",
                "--template",
                "vanilla-ts",
                "--yes",
            ],
        ),
        "bun" => (
            "bun",
            vec![
                "create",
                "tauri-app",
                ".",
                "--template",
                "vanilla-ts",
                "--yes",
            ],
        ),
        _ => (
            "npm",
            vec![
                "create",
                "tauri-app",
                ".",
                "--yes",
                "--",
                "--template",
                "vanilla-ts",
            ],
        ),
    };
    run_in(base, prog, &args, tx)
}

fn scaffold_electron(
    _params: &ScaffoldParams,
    base: &Path,
    pm: &str,
    tx: &Sender<String>,
) -> Result<(), String> {
    scaffold_emit(tx, "Running create-electron-vite...");
    let (prog, args): (&str, Vec<&str>) = match pm {
        "pnpm" => (
            "pnpm",
            vec!["create", "electron-vite", ".", "--template", "vanilla-ts"],
        ),
        "yarn" => (
            "yarn",
            vec!["create", "electron-vite", ".", "--template", "vanilla-ts"],
        ),
        "bun" => (
            "bun",
            vec!["create", "electron-vite", ".", "--template", "vanilla-ts"],
        ),
        _ => (
            "npm",
            vec![
                "create",
                "electron-vite",
                ".",
                "--",
                "--template",
                "vanilla-ts",
            ],
        ),
    };
    run_in(base, prog, &args, tx)
}

fn scaffold_neutralino(base: &Path, pm: &str, tx: &Sender<String>) -> Result<(), String> {
    scaffold_emit(tx, "Running npx neu create...");
    run_in(
        base,
        "npx",
        &["@neutralinojs/neu", "create", ".", "--template", "default"],
        tx,
    )?;

    scaffold_emit(tx, format!("Running {pm} install..."));
    let (prog, args): (&str, Vec<&str>) = match pm {
        "pnpm" => ("pnpm", vec!["install"]),
        "yarn" => ("yarn", vec!["install"]),
        "bun" => ("bun", vec!["install"]),
        _ => ("npm", vec!["install"]),
    };
    run_in(base, prog, &args, tx)
}

fn scaffold_emit(tx: &Sender<String>, msg: impl Into<String>) {
    let _ = tx.send(msg.into());
}
