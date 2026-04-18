.PHONY: all build test coverage clean run check help install

all: build

build:
	cargo build

release:
	cargo build --release

run:
	cargo run

check:
	cargo check 2>&1

test:
	cargo test

install:
	./scripts/install.sh

coverage:
	@command -v cargo-llvm-cov >/dev/null 2>&1 || (echo "cargo-llvm-cov is required. Install with: cargo install cargo-llvm-cov"; exit 1)
	@command -v jq >/dev/null 2>&1 || (echo "jq is required. Install it with your package manager (e.g. brew install jq)."; exit 1)
	@command -v column >/dev/null 2>&1 || (echo "column is required (usually provided by util-linux/bsdextrautils)."; exit 1)
	@tmp_file="$$(mktemp)"; \
	cargo llvm-cov --workspace --all-features --json --summary-only --output-path "$$tmp_file" -- --test-threads=1; \
	jq -r '"File\tLines %\tRegions %\tFunctions %", (.data[0].files[] | "\(.filename)\t\(.summary.lines.percent // 0)\t\(.summary.regions.percent // 0)\t\(.summary.functions.percent // 0)"), "TOTAL\t\(.data[0].totals.lines.percent // 0)\t\(.data[0].totals.regions.percent // 0)\t\(.data[0].totals.functions.percent // 0)"' "$$tmp_file" | column -t -s "$$(printf '\t')"; \
	rm -f "$$tmp_file"

clean:
	cargo clean

help:
	@echo "Usage: make <target>"
	@echo ""
	@echo "  build     Build (debug)"
	@echo "  release   Build (release)"
	@echo "  run       Run the TUI"
	@echo "  test      Run all tests"
	@echo "  coverage  Print per-file and total coverage (lines/regions/functions) to stdout"
	@echo "  clean     Remove build artifacts"
	@echo "  install   Install the binary and setup an alias"
	@echo "  help      Show this message"
