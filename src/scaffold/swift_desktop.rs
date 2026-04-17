use std::fs;
use std::path::Path;
use std::sync::mpsc::Sender;

use super::params::ScaffoldParams;
use super::writer::write_file;

pub fn scaffold(params: &ScaffoldParams, base: &Path, tx: &Sender<String>) -> Result<(), String> {
    let ui = params.sel("UI Framework").unwrap_or("SwiftUI");
    let core = params.sel("Core Language").unwrap_or("Swift Only");

    let _ = tx.send("Writing Swift desktop starter...".to_string());

    fs::create_dir_all(base.join("Sources"))
        .map_err(|e| format!("Failed to create Sources/: {e}"))?;

    write_file(base, "README.md", &readme(ui, core))?;
    write_file(base, "Package.swift", package_swift(ui))?;
    write_file(base, "Sources/main.swift", &entry(ui, core))?;

    Ok(())
}

fn readme(ui: &str, core: &str) -> String {
    format!("# Swift Desktop\n\n- UI Framework: {ui}\n- Core Language: {core}\n")
}

fn package_swift(ui: &str) -> &'static str {
    match ui {
        "AppKit" => {
            r#"// swift-tools-version: 6.0
import PackageDescription

let package = Package(
    name: "DesktopApp",
    platforms: [.macOS(.v15)],
    targets: [
        .executableTarget(name: "DesktopApp")
    ]
)
"#
        }
        _ => {
            r#"// swift-tools-version: 6.0
import PackageDescription

let package = Package(
    name: "DesktopApp",
    platforms: [.macOS(.v15)],
    targets: [
        .executableTarget(name: "DesktopApp")
    ]
)
"#
        }
    }
}

fn entry(ui: &str, core: &str) -> String {
    match (ui, core) {
        ("SwiftUI", "Swift + Rust") => r#"import SwiftUI

@main
struct DesktopApp: App {
    var body: some Scene {
        WindowGroup {
            Text("SwiftUI + Rust placeholder")
        }
    }
}
"#
        .to_string(),
        ("AppKit", _) => r#"import AppKit

print("AppKit desktop starter")
"#
        .to_string(),
        _ => r#"import SwiftUI

@main
struct DesktopApp: App {
    var body: some Scene {
        WindowGroup {
            Text("Hello Swift Desktop")
        }
    }
}
"#
        .to_string(),
    }
}
