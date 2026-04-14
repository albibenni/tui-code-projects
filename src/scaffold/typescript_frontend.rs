use std::path::PathBuf;

use crate::app::App;

use super::command::run_in;

pub fn scaffold(app: &App, base: &PathBuf) -> Result<(), String> {
    let framework = sel(app, "Framework").unwrap_or("");
    let variant   = sel(app, "Variant");
    let pm        = sel(app, "Package Manager").unwrap_or("npm");

    match framework {
        "React"   => scaffold_react(base, variant, pm),
        "Vue"     => scaffold_vite(base, "vue-ts", pm),
        "Svelte"  => scaffold_svelte(base, variant, pm),
        "Angular" => scaffold_angular(base, pm),
        "Astro"   => scaffold_astro(base, pm),
        "Qwik"    => scaffold_qwik(base, pm),
        "Solid"   => scaffold_vite(base, "solid-ts", pm),
        _         => Ok(()),
    }
}

fn scaffold_react(base: &PathBuf, variant: Option<&str>, pm: &str) -> Result<(), String> {
    match variant {
        Some("Next.js") => {
            run_in(base, "npx", &[
                "create-next-app@latest", ".",
                "--typescript",
                "--no-eslint",
                "--no-tailwind",
                "--no-src-dir",
                "--no-app",
                &format!("--use-{pm}"),
            ])
        }
        Some("Remix") => {
            run_in(base, "npx", &["create-remix@latest", ".", "--no-install"])
        }
        Some("Expo") => {
            run_in(base, "npx", &["create-expo-app@latest", ".", "--template", "blank-typescript"])
        }
        _ => scaffold_vite(base, "react-ts", pm),
    }
}

fn scaffold_svelte(base: &PathBuf, variant: Option<&str>, pm: &str) -> Result<(), String> {
    match variant {
        Some("SvelteKit") => {
            run_in(base, "npx", &[
                "sv@latest", "create", ".",
                "--template", "minimal",
                "--types", "typescript",
                "--no-add-ons",
            ])
        }
        _ => scaffold_vite(base, "svelte-ts", pm),
    }
}

fn scaffold_vite(base: &PathBuf, template: &str, pm: &str) -> Result<(), String> {
    let (prog, args): (&str, Vec<&str>) = match pm {
        "pnpm" => ("pnpm", vec!["create", "vite@latest", ".", "--template", template]),
        "yarn" => ("yarn", vec!["create", "vite", ".", "--template", template]),
        "bun"  => ("bun", vec!["create", "vite@latest", ".", "--template", template]),
        _      => ("npm", vec!["create", "vite@latest", ".", "--", "--template", template]),
    };
    run_in(base, prog, &args)
}

fn scaffold_angular(base: &PathBuf, pm: &str) -> Result<(), String> {
    run_in(base, "npx", &[
        "@angular/cli@latest", "new", ".",
        "--defaults",
        &format!("--package-manager={pm}"),
    ])
}

fn scaffold_astro(base: &PathBuf, pm: &str) -> Result<(), String> {
    let (prog, args): (&str, Vec<&str>) = match pm {
        "pnpm" => ("pnpm", vec!["create", "astro@latest", ".", "--template", "minimal", "--no-install", "--no-git"]),
        "yarn" => ("yarn", vec!["create", "astro", ".", "--template", "minimal", "--no-install", "--no-git"]),
        "bun"  => ("bun", vec!["create", "astro@latest", ".", "--template", "minimal", "--no-install", "--no-git"]),
        _      => ("npm", vec!["create", "astro@latest", ".", "--", "--template", "minimal", "--no-install", "--no-git"]),
    };
    run_in(base, prog, &args)
}

fn scaffold_qwik(base: &PathBuf, pm: &str) -> Result<(), String> {
    let (prog, args): (&str, Vec<&str>) = match pm {
        "pnpm" => ("pnpm", vec!["create", "qwik@latest", ".", "--no-install"]),
        "yarn" => ("yarn", vec!["create", "qwik", ".", "--no-install"]),
        "bun"  => ("bun", vec!["create", "qwik@latest", ".", "--no-install"]),
        _      => ("npm", vec!["create", "qwik@latest", ".", "--", "--no-install"]),
    };
    run_in(base, prog, &args)
}

fn sel<'a>(app: &'a App, title: &str) -> Option<&'a str> {
    app.option_selections
        .iter()
        .find(|s| s.title == title)
        .map(|s| s.choice_name)
}
