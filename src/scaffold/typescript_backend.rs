use std::path::PathBuf;

use crate::app::App;

use super::command::run_in;
use super::writer;

pub fn scaffold(app: &App, base: &PathBuf) -> Result<(), String> {
    let runtime  = sel(app, "Runtime").unwrap_or("Node");
    let framework = sel(app, "Framework").unwrap_or("");
    let pm       = sel(app, "Package Manager").unwrap_or("npm");
    let eslint   = sel(app, "ESLint").unwrap_or("None");

    if runtime == "Deno" {
        scaffold_deno(app, base, framework)
    } else {
        scaffold_node_bun(app, base, framework, pm, eslint)
    }
}

fn scaffold_deno(app: &App, base: &PathBuf, framework: &str) -> Result<(), String> {
    let imports = match framework {
        "Oak"  => "\n    \"oak\": \"jsr:@oak/oak\"",
        "Fresh" => "\n    \"$fresh/\": \"jsr:@fresh/fresh/\"",
        "Hono" => "\n    \"hono\": \"jsr:@hono/hono\"",
        _      => "",
    };

    let deno_json = format!(
        r#"{{
  "name": "{}",
  "version": "0.1.0",
  "tasks": {{
    "dev": "deno run --watch src/main.ts",
    "start": "deno run src/main.ts"
  }},
  "imports": {{{imports}
  }}
}}
"#,
        app.config.project_name
    );

    writer::write_file(base, "deno.json", &deno_json)
}

fn scaffold_node_bun(
    app: &App,
    base: &PathBuf,
    framework: &str,
    pm: &str,
    eslint: &str,
) -> Result<(), String> {
    let name = &app.config.project_name;

    let mut deps: Vec<&str>     = Vec::new();
    let mut dev_deps: Vec<&str> = Vec::new();

    match framework {
        "Express" => {
            deps.push("express");
            dev_deps.push("@types/express");
        }
        "Fastify" => {
            deps.push("fastify");
        }
        "NestJS" => {
            deps.extend_from_slice(&[
                "@nestjs/core",
                "@nestjs/common",
                "rxjs",
                "reflect-metadata",
            ]);
            dev_deps.push("@nestjs/cli");
        }
        "Hono" | "Elysia" => {
            deps.push(framework.to_lowercase().leak());
        }
        _ => {}
    }

    dev_deps.extend_from_slice(&["typescript", "@types/node", "tsx"]);

    match eslint {
        "Recommended" => {
            dev_deps.extend_from_slice(&["eslint", "@eslint/js", "typescript-eslint", "globals"]);
        }
        "Recommended + Prettier" => {
            dev_deps.extend_from_slice(&[
                "eslint",
                "@eslint/js",
                "typescript-eslint",
                "globals",
                "prettier",
                "eslint-plugin-prettier",
                "eslint-config-prettier",
            ]);
        }
        "Custom Strict" => {
            dev_deps.extend_from_slice(&[
                "eslint",
                "@eslint/js",
                "@eslint/eslintrc",
                "typescript-eslint",
                "globals",
                "eslint-plugin-prettier",
                "eslint-config-prettier",
            ]);
        }
        _ => {}
    }

    let deps_lines: String = deps
        .iter()
        .map(|d| format!("    \"{d}\": \"latest\""))
        .collect::<Vec<_>>()
        .join(",\n");

    let dev_lines: String = dev_deps
        .iter()
        .map(|d| format!("    \"{d}\": \"latest\""))
        .collect::<Vec<_>>()
        .join(",\n");

    let deps_block = if deps.is_empty() {
        "  \"dependencies\": {},".to_string()
    } else {
        format!("  \"dependencies\": {{\n{deps_lines}\n  }},")
    };

    let package_json = format!(
        r#"{{
  "name": "{name}",
  "version": "0.1.0",
  "type": "module",
  "scripts": {{
    "dev": "tsx watch src/index.ts",
    "build": "tsc",
    "start": "node dist/index.js"
  }},
{deps_block}
  "devDependencies": {{
{dev_lines}
  }}
}}
"#
    );

    writer::write_file(base, "package.json", &package_json)?;

    let (prog, args): (&str, &[&str]) = match pm {
        "pnpm" => ("pnpm", &["install"]),
        "yarn" => ("yarn", &[]),
        "bun"  => ("bun", &["install"]),
        _      => ("npm", &["install"]),
    };
    run_in(base, prog, args)?;

    writer::write_eslint_files(base, eslint)
}

fn sel<'a>(app: &'a App, title: &str) -> Option<&'a str> {
    app.option_selections
        .iter()
        .find(|s| s.title == title)
        .map(|s| s.choice_name)
}
