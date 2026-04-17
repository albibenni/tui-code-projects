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

    Ok(())
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
