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
    write_file(base, "Makefile", &makefile(&params.project_name))?;

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

fn makefile(project_name: &str) -> String {
    let test_bundle = format!("{project_name}PackageTests");
    let template = "SWIFT ?= swift

.PHONY: build run test fmt lint coverage

build:
\t@$(SWIFT) build

run:
\t@$(SWIFT) run

test:
\t@$(SWIFT) test

fmt:
\t@$(SWIFT) format --in-place --recursive Sources Tests || true

lint:
\t@$(SWIFT) format lint --recursive Sources Tests || true

coverage:
\t@rm -rf .build/coverage-home .build/coverage-main .build/coverage-merged
\t@mkdir -p .build/coverage-home .build/coverage-merged
\t@HOME=$$PWD/.build/coverage-home \\
\tXDG_CONFIG_HOME=$$PWD/.build/coverage-home \\
\tFREE_COVERAGE_MODE=1 \\
\t\t$(SWIFT) test --enable-code-coverage --no-parallel \\
\t\t--scratch-path .build/coverage-main
\t@profraw_count=$$(find .build/coverage-main -name \"*.profraw\" | wc -l | tr -d ' '); \\
\tbin=$$(find .build/coverage-main -path \"*/debug/__TEST_BUNDLE__.xctest/Contents/MacOS/__TEST_BUNDLE__\" -not -path \"*.dSYM/*\" | head -n 1); \\
\tsrc_files=$$(find Sources -type f -name \"*.swift\" | sort); \\
\tif [[ \"$$profraw_count\" == \"0\" || -z \"$$bin\" ]]; then \\
\t\techo \"Could not locate coverage artifacts.\"; \\
\t\texit 1; \\
\tfi; \\
\tif [[ -z \"$$src_files\" ]]; then \\
\t\techo \"Could not locate source files for coverage report.\"; \\
\t\texit 1; \\
\tfi; \\
\tfind .build/coverage-main -name \"*.profraw\" -print0 \\
\t\t| xargs -0 xcrun llvm-profdata merge -sparse -o .build/coverage-merged/merged.profdata; \\
\txcrun llvm-cov report \"$$bin\" -instr-profile=.build/coverage-merged/merged.profdata $$src_files
";
    template.replace("__TEST_BUNDLE__", &test_bundle)
}
