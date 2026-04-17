use std::fs;
use std::path::Path;
use std::sync::mpsc::Sender;

use super::params::ScaffoldParams;
use super::writer::write_file;

pub fn scaffold(params: &ScaffoldParams, base: &Path, tx: &Sender<String>) -> Result<(), String> {
    let framework = params.sel("Framework").unwrap_or("Fyne");

    let _ = tx.send("Writing Go desktop starter...".to_string());
    fs::create_dir_all(base).map_err(|e| format!("Failed to create project directory: {e}"))?;

    write_file(base, "README.md", &readme(framework))?;
    write_file(base, "go.mod", go_mod())?;
    write_file(base, "main.go", main_go(framework))?;
    write_file(base, "Makefile", makefile())?;

    Ok(())
}

fn makefile() -> &'static str {
    r#"GO ?= go

.PHONY: run build test fmt lint tidy

run:
	@$(GO) run .

build:
	@$(GO) build .

test:
	@$(GO) test ./...

fmt:
	@$(GO) fmt ./...

lint:
	@$(GO) vet ./...

tidy:
	@$(GO) mod tidy
"#
}

fn readme(framework: &str) -> String {
    format!("# Go Desktop\n\n- Framework: {framework}\n")
}

fn go_mod() -> &'static str {
    r#"module desktopapp

go 1.23
"#
}

fn main_go(framework: &str) -> &'static str {
    match framework {
        "Gio" => {
            r#"package main

import "fmt"

func main() {
	fmt.Println("Gio desktop starter")
}
"#
        }
        _ => {
            r#"package main

import "fmt"

func main() {
	fmt.Println("Fyne desktop starter")
}
"#
        }
    }
}
