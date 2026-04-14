use std::path::PathBuf;
use std::sync::mpsc::Sender;

use super::command::run_in;
use super::params::ScaffoldParams;
use super::writer;

pub fn scaffold(params: &ScaffoldParams, base: &PathBuf, tx: &Sender<String>) -> Result<(), String> {
    let runtime   = params.sel("Runtime").unwrap_or("Node");
    let framework = params.sel("Framework").unwrap_or("");
    let pm        = params.sel("Package Manager").unwrap_or("npm");
    let eslint    = params.sel("ESLint").unwrap_or("None");

    if runtime == "Deno" {
        scaffold_deno(params, base, framework, tx)
    } else {
        scaffold_node_bun(params, base, framework, pm, eslint, tx)
    }
}

fn scaffold_deno(
    params: &ScaffoldParams,
    base: &PathBuf,
    framework: &str,
    tx: &Sender<String>,
) -> Result<(), String> {
    let imports = match framework {
        "Oak"   => "\n    \"oak\": \"jsr:@oak/oak\"",
        "Fresh" => "\n    \"$fresh/\": \"jsr:@fresh/fresh/\"",
        "Hono"  => "\n    \"hono\": \"jsr:@hono/hono\"",
        _       => "",
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
        params.project_name
    );

    send(tx, "Writing deno.json...");
    writer::write_file(base, "deno.json", &deno_json)
}

fn scaffold_node_bun(
    params: &ScaffoldParams,
    base: &PathBuf,
    framework: &str,
    pm: &str,
    eslint: &str,
    tx: &Sender<String>,
) -> Result<(), String> {
    let name = &params.project_name;

    let mut deps: Vec<&str>     = Vec::new();
    let mut dev_deps: Vec<&str> = Vec::new();

    match framework {
        "Express" => {
            deps.push("express");
            dev_deps.push("@types/express");
        }
        "Fastify" => { deps.push("fastify"); }
        "NestJS"  => {
            deps.extend_from_slice(&["@nestjs/core", "@nestjs/common", "rxjs", "reflect-metadata"]);
            dev_deps.push("@nestjs/cli");
        }
        "Hono"   => { deps.push("hono"); }
        "Elysia" => { deps.push("elysia"); }
        _        => {}
    }

    dev_deps.extend_from_slice(&["typescript", "@types/node", "tsx"]);

    match eslint {
        "Recommended" => {
            dev_deps.extend_from_slice(&["eslint", "@eslint/js", "typescript-eslint", "globals"]);
        }
        "Recommended + Prettier" => {
            dev_deps.extend_from_slice(&[
                "eslint", "@eslint/js", "typescript-eslint", "globals",
                "prettier", "eslint-plugin-prettier", "eslint-config-prettier",
            ]);
        }
        "Custom Strict" => {
            dev_deps.extend_from_slice(&[
                "eslint", "@eslint/js", "@eslint/eslintrc",
                "typescript-eslint", "globals",
                "eslint-plugin-prettier", "eslint-config-prettier",
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

    send(tx, "Writing package.json...");
    writer::write_file(base, "package.json", &package_json)?;

    send(tx, format!("Running {pm} install..."));
    let (prog, args): (&str, &[&str]) = match pm {
        "pnpm" => ("pnpm", &["install"]),
        "yarn" => ("yarn", &[]),
        "bun"  => ("bun", &["install"]),
        _      => ("npm", &["install"]),
    };
    run_in(base, prog, args, tx)?;

    send(tx, "Writing config files...");
    writer::write_eslint_files(base, eslint)
}

fn send(tx: &Sender<String>, msg: impl Into<String>) {
    let _ = tx.send(msg.into());
}
