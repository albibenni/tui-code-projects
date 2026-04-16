use std::path::Path;
use std::sync::mpsc::Sender;

use super::command::run_in;
use super::params::ScaffoldParams;

pub fn scaffold(params: &ScaffoldParams, base: &Path, tx: &Sender<String>) -> Result<(), String> {
    let framework = params.sel("Framework").unwrap_or("");
    let variant   = params.sel("Variant");
    let pm        = params.sel("Package Manager").unwrap_or("npm");

    match framework {
        "React"   => scaffold_react(base, variant, pm, tx),
        "Vue"     => scaffold_vite(base, "vue-ts", pm, tx),
        "Svelte"  => scaffold_svelte(base, variant, pm, tx),
        "Angular" => scaffold_angular(base, pm, tx),
        "Astro"   => scaffold_astro(base, pm, tx),
        "Qwik"    => scaffold_qwik(base, pm, tx),
        "Solid"   => scaffold_vite(base, "solid-ts", pm, tx),
        _         => Ok(()),
    }
}

fn scaffold_react(base: &Path, variant: Option<&str>, pm: &str, tx: &Sender<String>) -> Result<(), String> {
    match variant {
        Some("Next.js") => {
            send(tx, "Running create-next-app...");
            run_in(base, "npx", &[
                "create-next-app@latest", ".",
                "--typescript", "--no-eslint", "--no-tailwind",
                "--no-src-dir", "--no-app",
                &format!("--use-{pm}"),
            ], tx)
        }
        Some("Remix") => {
            send(tx, "Running create-remix...");
            run_in(base, "npx", &["create-remix@latest", ".", "--no-install"], tx)
        }
        Some("Expo") => {
            send(tx, "Running create-expo-app...");
            run_in(base, "npx", &["create-expo-app@latest", ".", "--template", "blank-typescript"], tx)
        }
        _ => scaffold_vite(base, "react-ts", pm, tx),
    }
}

fn scaffold_svelte(base: &Path, variant: Option<&str>, pm: &str, tx: &Sender<String>) -> Result<(), String> {
    match variant {
        Some("SvelteKit") => {
            send(tx, "Running sv create...");
            run_in(base, "npx", &[
                "sv@latest", "create", ".",
                "--template", "minimal",
                "--types", "typescript",
                "--no-add-ons",
            ], tx)
        }
        _ => scaffold_vite(base, "svelte-ts", pm, tx),
    }
}

fn scaffold_vite(base: &Path, template: &str, pm: &str, tx: &Sender<String>) -> Result<(), String> {
    send(tx, format!("Running create-vite ({template})..."));
    let (prog, args): (&str, Vec<&str>) = match pm {
        "pnpm" => ("pnpm", vec!["create", "vite@latest", ".", "--template", template]),
        "yarn" => ("yarn", vec!["create", "vite", ".", "--template", template]),
        "bun"  => ("bun", vec!["create", "vite@latest", ".", "--template", template]),
        _      => ("npm", vec!["create", "vite@latest", ".", "--", "--template", template]),
    };
    run_in(base, prog, &args, tx)
}

fn scaffold_angular(base: &Path, pm: &str, tx: &Sender<String>) -> Result<(), String> {
    send(tx, "Running @angular/cli new...");
    run_in(base, "npx", &[
        "@angular/cli@latest", "new", ".",
        "--defaults",
        &format!("--package-manager={pm}"),
    ], tx)
}

fn scaffold_astro(base: &Path, pm: &str, tx: &Sender<String>) -> Result<(), String> {
    send(tx, "Running create-astro...");
    let (prog, args): (&str, Vec<&str>) = match pm {
        "pnpm" => ("pnpm", vec!["create", "astro@latest", ".", "--template", "minimal", "--no-install", "--no-git"]),
        "yarn" => ("yarn", vec!["create", "astro", ".", "--template", "minimal", "--no-install", "--no-git"]),
        "bun"  => ("bun", vec!["create", "astro@latest", ".", "--template", "minimal", "--no-install", "--no-git"]),
        _      => ("npm", vec!["create", "astro@latest", ".", "--", "--template", "minimal", "--no-install", "--no-git"]),
    };
    run_in(base, prog, &args, tx)
}

fn scaffold_qwik(base: &Path, pm: &str, tx: &Sender<String>) -> Result<(), String> {
    send(tx, "Running create-qwik...");
    let (prog, args): (&str, Vec<&str>) = match pm {
        "pnpm" => ("pnpm", vec!["create", "qwik@latest", ".", "--no-install"]),
        "yarn" => ("yarn", vec!["create", "qwik", ".", "--no-install"]),
        "bun"  => ("bun", vec!["create", "qwik@latest", ".", "--no-install"]),
        _      => ("npm", vec!["create", "qwik@latest", ".", "--", "--no-install"]),
    };
    run_in(base, prog, &args, tx)
}

fn send(tx: &Sender<String>, msg: impl Into<String>) {
    let _ = tx.send(msg.into());
}
