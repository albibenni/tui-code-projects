use std::fs;
use std::path::Path;
use std::sync::mpsc::Sender;

use super::params::ScaffoldParams;
use super::writer::write_file;

pub fn scaffold(params: &ScaffoldParams, base: &Path, tx: &Sender<String>) -> Result<(), String> {
    let framework = params.sel("Framework").unwrap_or("Tauri");

    let _ = tx.send("Writing Rust desktop starter...".to_string());
    fs::create_dir_all(base.join("src")).map_err(|e| format!("Failed to create src/: {e}"))?;

    write_file(base, "README.md", &readme(framework))?;
    write_file(base, "Cargo.toml", cargo_toml(framework))?;
    write_file(base, "src/main.rs", main_rs(framework))?;
    write_file(base, "Makefile", makefile())?;

    Ok(())
}

fn makefile() -> &'static str {
    r#"CARGO ?= cargo

.PHONY: build run test fmt lint clippy coverage

build:
	@$(CARGO) build

run:
	@$(CARGO) run

test:
	@$(CARGO) test

fmt:
	@$(CARGO) fmt

lint:
	@$(CARGO) clippy --all-targets --all-features -- -D warnings

clippy:
	@$(CARGO) clippy --all-targets --all-features -- -D warnings

coverage:
	@command -v cargo-llvm-cov >/dev/null 2>&1 || (echo "cargo-llvm-cov is required. Install with: cargo install cargo-llvm-cov"; exit 1)
	@command -v jq >/dev/null 2>&1 || (echo "jq is required. Install it with your package manager (e.g. brew install jq)."; exit 1)
	@command -v column >/dev/null 2>&1 || (echo "column is required (usually provided by util-linux/bsdextrautils)."; exit 1)
	@tmp_file="$$(mktemp)"; \
	cargo llvm-cov --workspace --all-features --json --summary-only --output-path "$$tmp_file" -- --test-threads=1; \
	jq -r '"File\tLines %\tRegions %\tFunctions %", (.data[0].files[] | "\(.filename)\t\(.summary.lines.percent // 0)\t\(.summary.regions.percent // 0)\t\(.summary.functions.percent // 0)"), "TOTAL\t\(.data[0].totals.lines.percent // 0)\t\(.data[0].totals.regions.percent // 0)\t\(.data[0].totals.functions.percent // 0)"' "$$tmp_file" | column -t -s "$$(printf '\t')"; \
	rm -f "$$tmp_file"
"#
}

fn readme(framework: &str) -> String {
    format!("# Rust Desktop\n\n- Framework: {framework}\n")
}

fn cargo_toml(framework: &str) -> &'static str {
    match framework {
        "gtk4-rs" => {
            r#"[package]
name = "desktop_app"
version = "0.1.0"
edition = "2021"

[dependencies]
gtk4 = "0.9"
"#
        }
        _ => {
            r#"[package]
name = "desktop_app"
version = "0.1.0"
edition = "2021"

[dependencies]
"#
        }
    }
}

fn main_rs(framework: &str) -> &'static str {
    match framework {
        "gtk4-rs" => {
            r#"use gtk4 as gtk;
use gtk::prelude::*;

fn main() {
    let app = gtk::Application::builder()
        .application_id("com.example.desktop")
        .build();

    app.connect_activate(|app| {
        let win = gtk::ApplicationWindow::builder()
            .application(app)
            .title("GTK4 App")
            .default_width(640)
            .default_height(360)
            .build();
        win.present();
    });

    app.run();
}
"#
        }
        _ => {
            r#"fn main() {
    println!("Rust desktop starter (Tauri placeholder)");
}
"#
        }
    }
}
