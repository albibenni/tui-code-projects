use std::path::Path;
use std::sync::mpsc::Sender;

use super::command::run_in;
use super::params::ScaffoldParams;
use super::writer::write_file;

pub fn scaffold(params: &ScaffoldParams, base: &Path, tx: &Sender<String>) -> Result<(), String> {
    let project_type = params.sel("Project Type").unwrap_or("Binary");

    if project_type == "Library" {
        let _ = tx.send("Running cargo init --lib...".to_string());
        run_in(base, "cargo", &["init", "--lib"], tx)?;
    } else {
        let _ = tx.send("Running cargo init...".to_string());
        run_in(base, "cargo", &["init"], tx)?;
    }

    write_file(base, "Makefile", makefile())
}

fn makefile() -> &'static str {
    r#"CARGO ?= cargo

.PHONY: build run test fmt clippy coverage

build:
	@$(CARGO) build

run:
	@$(CARGO) run

test:
	@$(CARGO) test

fmt:
	@$(CARGO) fmt

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
