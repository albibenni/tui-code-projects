use std::fs;
use std::path::Path;
use std::sync::mpsc::Sender;

use super::params::ScaffoldParams;
use super::writer::write_file;

pub fn scaffold(params: &ScaffoldParams, base: &Path, tx: &Sender<String>) -> Result<(), String> {
    let toolkit = params.sel("UI Toolkit").unwrap_or("Jetpack Compose");
    let build_tool = params.sel("Build Tool").unwrap_or("Gradle (KTS)");

    let _ = tx.send("Writing Kotlin mobile starter...".to_string());

    fs::create_dir_all(base.join("app/src/main/kotlin/com/example/app"))
        .map_err(|e| format!("Failed to create source directories: {e}"))?;

    write_file(base, "README.md", &readme(toolkit, build_tool))?;
    write_file(
        base,
        "settings.gradle.kts",
        "rootProject.name = \"mobile-app\"\n",
    )?;
    write_file(
        base,
        "app/src/main/kotlin/com/example/app/Main.kt",
        main_kt(toolkit),
    )?;
    write_file(base, "build.gradle.kts", build_gradle_kts(toolkit))?;

    Ok(())
}

fn readme(toolkit: &str, build_tool: &str) -> String {
    format!("# Kotlin Mobile\n\n- UI Toolkit: {toolkit}\n- Build Tool: {build_tool}\n")
}

fn build_gradle_kts(toolkit: &str) -> &'static str {
    match toolkit {
        "XML Views" => {
            r#"plugins {
    kotlin("android") version "2.1.0" apply false
}

// Placeholder root Gradle script for XML Views project setup.
"#
        }
        _ => {
            r#"plugins {
    kotlin("android") version "2.1.0" apply false
}

// Placeholder root Gradle script for Jetpack Compose project setup.
"#
        }
    }
}

fn main_kt(toolkit: &str) -> &'static str {
    match toolkit {
        "XML Views" => {
            r#"package com.example.app

fun main() {
    println("Kotlin Android starter (XML Views)")
}
"#
        }
        _ => {
            r#"package com.example.app

fun main() {
    println("Kotlin Android starter (Jetpack Compose)")
}
"#
        }
    }
}
