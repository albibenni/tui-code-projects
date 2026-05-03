use std::path::Path;
use std::sync::mpsc::Sender;

use super::command::run_in;
use super::params::ScaffoldParams;
use super::writer;

pub fn scaffold(params: &ScaffoldParams, base: &Path, tx: &Sender<String>) -> Result<(), String> {
    let framework = params.sel("Framework").unwrap_or("");
    let variant = params.sel("Variant");
    let pm = params.sel("Package Manager").unwrap_or("npm");
    let eslint = params.sel("ESLint").unwrap_or("Recommended + Prettier");
    let libraries = params.sel("Libraries").unwrap_or("None");
    let testing = params.sel("Testing").unwrap_or("None");

    match framework {
        "React" => scaffold_react(base, variant, pm, tx),
        "Vue" => scaffold_vue(base, variant, pm, tx),
        "Svelte" => scaffold_svelte(base, variant, pm, tx),
        "Angular" => scaffold_angular(base, pm, tx),
        "Astro" => scaffold_astro(base, pm, tx),
        "Qwik" => scaffold_qwik(base, pm, tx),
        "Solid" => scaffold_vite(base, "solid-ts", pm, tx),
        _ => Ok(()),
    }?;

    if framework == "React" && libraries != "None" {
        if let Err(e) = setup_libraries(base, pm, libraries, tx) {
            let _ = tx.send(format!("Warning: library setup encountered an issue: {e}"));
        }
    }

    setup_default_eslint(base, pm, eslint, tx)?;
    setup_testing(base, pm, testing, tx)?;
    writer::ensure_js_linting_scripts(base, eslint)
}

fn setup_libraries(
    base: &Path,
    pm: &str,
    libraries: &str,
    tx: &Sender<String>,
) -> Result<(), String> {
    let mut deps = Vec::new();
    let mut dev_deps = Vec::new();

    if libraries.contains("TanStack Query") {
        deps.push("@tanstack/react-query");
        dev_deps.push("@tanstack/eslint-plugin-query");
    }

    if libraries.contains("Tailwind CSS") {
        dev_deps.extend_from_slice(&["tailwindcss", "postcss", "autoprefixer"]);
    }

    if libraries.contains("Lucide React") {
        deps.push("lucide-react");
    }

    if !deps.is_empty() {
        scaffold_emit(tx, format!("Installing libraries: {}...", deps.join(", ")));
        let (prog, mut args) = add_command(pm, false);
        args.extend_from_slice(&deps);
        run_in(base, prog, &args, tx)?;
    }

    if !dev_deps.is_empty() {
        scaffold_emit(
            tx,
            format!("Installing dev libraries: {}...", dev_deps.join(", ")),
        );
        let (prog, mut args) = add_command(pm, true);
        args.extend_from_slice(&dev_deps);
        run_in(base, prog, &args, tx)?;
    }

    if libraries.contains("Tailwind CSS") {
        scaffold_emit(tx, "Initializing Tailwind CSS...");
        let (prog, args) = match pm {
            "pnpm" => ("pnpm", vec!["tailwindcss", "init", "-p"]),
            "yarn" => ("yarn", vec!["tailwindcss", "init", "-p"]),
            "bun" => ("bun", vec!["x", "tailwindcss", "init", "-p"]),
            _ => ("npx", vec!["--yes", "tailwindcss", "init", "-p"]),
        };
        if let Err(e) = run_in(base, prog, &args, tx) {
            let _ = tx.send(format!("Warning: failed to initialize Tailwind CSS: {e}"));
        }
    }

    Ok(())
}

fn add_command(pm: &str, dev: bool) -> (&str, Vec<&'static str>) {
    match (pm, dev) {
        ("pnpm", true) => ("pnpm", vec!["add", "-D"]),
        ("pnpm", false) => ("pnpm", vec!["add"]),
        ("yarn", true) => ("yarn", vec!["add", "-D"]),
        ("yarn", false) => ("yarn", vec!["add"]),
        ("bun", true) => ("bun", vec!["add", "-d"]),
        ("bun", false) => ("bun", vec!["add"]),
        (_, true) => ("npm", vec!["install", "-D"]),
        (_, false) => ("npm", vec!["install"]),
    }
}

fn scaffold_vue(
    base: &Path,
    variant: Option<&str>,
    pm: &str,
    tx: &Sender<String>,
) -> Result<(), String> {
    match variant {
        Some("Nuxt") => {
            scaffold_emit(tx, "Running nuxi init...");
            run_in(base, "npx", &["nuxi@latest", "init", ".", "--force"], tx)?;
            install_deps(base, pm, tx)
        }
        _ => scaffold_vite(base, "vue-ts", pm, tx),
    }
}

fn scaffold_react(
    base: &Path,
    variant: Option<&str>,
    pm: &str,
    tx: &Sender<String>,
) -> Result<(), String> {
    match variant {
        Some("Next.js") => {
            scaffold_emit(tx, "Running create-next-app...");
            run_in(
                base,
                "npx",
                &[
                    "create-next-app@latest",
                    ".",
                    "--typescript",
                    "--eslint",
                    "--no-tailwind",
                    "--no-src-dir",
                    "--no-app",
                    &format!("--use-{pm}"),
                    "--yes",
                ],
                tx,
            )
        }
        Some("Remix") => {
            scaffold_emit(tx, "Running create-remix...");
            run_in(
                base,
                "npx",
                &["create-remix@latest", ".", "--no-install", "--yes"],
                tx,
            )?;
            install_deps(base, pm, tx)
        }
        Some("TanStack Start") => {
            scaffold_emit(tx, "Running tanstack create...");
            run_in(
                base,
                "npx",
                &[
                    "@tanstack/cli@latest",
                    "create",
                    ".",
                    "--package-manager",
                    pm,
                    "--non-interactive",
                    "--yes",
                    "--no-git",
                    "--force",
                ],
                tx,
            )
        }
        Some("Expo") => {
            scaffold_emit(tx, "Running create-expo-app...");
            run_in(
                base,
                "npx",
                &[
                    "create-expo-app@latest",
                    ".",
                    "--template",
                    "blank-typescript",
                    "--yes",
                ],
                tx,
            )
        }
        _ => scaffold_vite(base, "react-ts", pm, tx),
    }
}

fn scaffold_svelte(
    base: &Path,
    variant: Option<&str>,
    pm: &str,
    tx: &Sender<String>,
) -> Result<(), String> {
    match variant {
        Some("SvelteKit") => {
            scaffold_emit(tx, "Running sv create...");
            run_in(
                base,
                "npx",
                &[
                    "sv@latest",
                    "create",
                    ".",
                    "--template",
                    "minimal",
                    "--types",
                    "typescript",
                    "--no-add-ons",
                ],
                tx,
            )
        }
        _ => scaffold_vite(base, "svelte-ts", pm, tx),
    }
}

fn scaffold_vite(base: &Path, template: &str, pm: &str, tx: &Sender<String>) -> Result<(), String> {
    scaffold_emit(tx, format!("Running create-vite ({template})..."));
    let (prog, args): (&str, Vec<&str>) = match pm {
        "pnpm" => (
            "pnpm",
            vec!["create", "vite@latest", ".", "--template", template, "--yes"],
        ),
        "yarn" => ("yarn", vec!["create", "vite", ".", "--template", template, "--yes"]),
        "bun" => (
            "bun",
            vec!["create", "vite@latest", ".", "--template", template, "--yes"],
        ),
        _ => (
            "npm",
            vec![
                "create",
                "vite@latest",
                ".",
                "--yes",
                "--",
                "--template",
                template,
            ],
        ),
    };
    run_in(base, prog, &args, tx)?;
    install_deps(base, pm, tx)
}

fn scaffold_angular(base: &Path, pm: &str, tx: &Sender<String>) -> Result<(), String> {
    scaffold_emit(tx, "Running @angular/cli new...");
    run_in(
        base,
        "npx",
        &[
            "@angular/cli@latest",
            "new",
            ".",
            "--defaults",
            &format!("--package-manager={pm}"),
        ],
        tx,
    )
}

fn scaffold_astro(base: &Path, pm: &str, tx: &Sender<String>) -> Result<(), String> {
    scaffold_emit(tx, "Running create-astro...");
    let (prog, args): (&str, Vec<&str>) = match pm {
        "pnpm" => (
            "pnpm",
            vec![
                "create",
                "astro@latest",
                ".",
                "--template",
                "minimal",
                "--no-install",
                "--no-git",
                "--yes",
            ],
        ),
        "yarn" => (
            "yarn",
            vec![
                "create",
                "astro",
                ".",
                "--template",
                "minimal",
                "--no-install",
                "--no-git",
                "--yes",
            ],
        ),
        "bun" => (
            "bun",
            vec![
                "create",
                "astro@latest",
                ".",
                "--template",
                "minimal",
                "--no-install",
                "--no-git",
                "--yes",
            ],
        ),
        _ => (
            "npm",
            vec![
                "create",
                "astro@latest",
                ".",
                "--yes",
                "--",
                "--template",
                "minimal",
                "--no-install",
                "--no-git",
            ],
        ),
    };
    run_in(base, prog, &args, tx)?;
    install_deps(base, pm, tx)
}

fn scaffold_qwik(base: &Path, pm: &str, tx: &Sender<String>) -> Result<(), String> {
    scaffold_emit(tx, "Running create-qwik...");
    let (prog, args): (&str, Vec<&str>) = match pm {
        "pnpm" => (
            "pnpm",
            vec!["create", "qwik@latest", ".", "--no-install", "--yes"],
        ),
        "yarn" => ("yarn", vec!["create", "qwik", ".", "--no-install", "--yes"]),
        "bun" => (
            "bun",
            vec!["create", "qwik@latest", ".", "--no-install", "--yes"],
        ),
        _ => (
            "npm",
            vec!["create", "qwik@latest", ".", "--yes", "--", "--no-install"],
        ),
    };
    run_in(base, prog, &args, tx)?;
    install_deps(base, pm, tx)
}

fn install_deps(base: &Path, pm: &str, tx: &Sender<String>) -> Result<(), String> {
    let (prog, args) = install_command(pm);
    scaffold_emit(tx, format!("Installing dependencies ({pm})..."));
    run_in(base, prog, &args, tx)
}

fn install_command(pm: &str) -> (&str, Vec<&'static str>) {
    match pm {
        "pnpm" => ("pnpm", vec!["install"]),
        "yarn" => ("yarn", vec!["install"]),
        "bun" => ("bun", vec!["install"]),
        _ => ("npm", vec!["install"]),
    }
}

fn setup_default_eslint(
    base: &Path,
    pm: &str,
    eslint: &str,
    tx: &Sender<String>,
) -> Result<(), String> {
    let dev_deps = eslint_dev_deps(eslint);
    if dev_deps.is_empty() {
        return Ok(());
    }

    scaffold_emit(tx, format!("Installing ESLint dev dependencies ({pm})..."));
    let mut args: Vec<&str> = Vec::new();
    let prog = match pm {
        "pnpm" => {
            args.extend_from_slice(&["add", "-D"]);
            "pnpm"
        }
        "yarn" => {
            args.extend_from_slice(&["add", "-D"]);
            "yarn"
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
    args.extend_from_slice(dev_deps);
    run_in(base, prog, &args, tx)?;

    scaffold_emit(tx, "Writing ESLint/Prettier config files...");
    writer::write_eslint_config_files(base, eslint, writer::EslintTarget::Frontend)
}

fn eslint_dev_deps(eslint: &str) -> &'static [&'static str] {
    match eslint {
        "Recommended" => &["eslint", "@eslint/js", "typescript-eslint", "globals"],
        "Recommended + Prettier" => &[
            "eslint",
            "@eslint/js",
            "typescript-eslint",
            "globals",
            "prettier",
            "eslint-plugin-prettier",
            "eslint-config-prettier",
        ],
        "Custom Strict" => &[
            "eslint",
            "@eslint/js",
            "@eslint/eslintrc",
            "typescript-eslint",
            "globals",
            "prettier",
            "eslint-plugin-prettier",
            "eslint-config-prettier",
        ],
        _ => &[],
    }
}

fn setup_testing(base: &Path, pm: &str, testing: &str, tx: &Sender<String>) -> Result<(), String> {
    if testing != "Vitest" {
        return Ok(());
    }

    scaffold_emit(tx, format!("Installing Vitest dependencies ({pm})..."));
    let mut args: Vec<&str> = Vec::new();
    let prog = match pm {
        "pnpm" => {
            args.extend_from_slice(&["add", "-D"]);
            "pnpm"
        }
        "yarn" => {
            args.extend_from_slice(&["add", "-D"]);
            "yarn"
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

    // Update package.json scripts
    let scripts = &[
        ("test", "vitest run"),
        ("test:watch", "vitest"),
        ("test:coverage", "vitest run --coverage"),
    ];
    writer::ensure_package_json_scripts(base, scripts)?;

    Ok(())
}

fn scaffold_emit(tx: &Sender<String>, msg: impl Into<String>) {
    let _ = tx.send(msg.into());
}

#[cfg(test)]
mod tests {
    use super::install_command;

    #[test]
    fn install_command_maps_package_managers() {
        assert_eq!(install_command("npm"), ("npm", vec!["install"]));
        assert_eq!(install_command("pnpm"), ("pnpm", vec!["install"]));
        assert_eq!(install_command("yarn"), ("yarn", vec!["install"]));
        assert_eq!(install_command("bun"), ("bun", vec!["install"]));
    }
}
