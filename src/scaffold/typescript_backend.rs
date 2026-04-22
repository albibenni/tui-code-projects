use std::path::Path;
use std::sync::mpsc::Sender;

use super::command::run_in;
use super::params::ScaffoldParams;
use super::writer;

pub fn scaffold(params: &ScaffoldParams, base: &Path, tx: &Sender<String>) -> Result<(), String> {
    let runtime = params.sel("Runtime").unwrap_or("Node");
    let framework = params.sel("Framework").unwrap_or("");
    let pm = params.sel("Package Manager").unwrap_or("npm");
    let eslint = params.sel("ESLint").unwrap_or("None");

    if runtime == "Deno" {
        scaffold_deno(params, base, framework, tx)
    } else {
        scaffold_node_bun(params, base, framework, pm, eslint, tx)
    }
}

fn scaffold_deno(
    params: &ScaffoldParams,
    base: &Path,
    framework: &str,
    tx: &Sender<String>,
) -> Result<(), String> {
    let mut imports = serde_json::Map::new();
    match framework {
        "Oak" => {
            imports.insert("oak".to_string(), serde_json::json!("jsr:@oak/oak"));
        }
        "Fresh" => {
            imports.insert("$fresh/".to_string(), serde_json::json!("jsr:@fresh/fresh/"));
        }
        "Hono" => {
            imports.insert("hono".to_string(), serde_json::json!("jsr:@hono/hono"));
        }
        _ => {}
    };

    let deno_json_val = serde_json::json!({
        "name": params.project_name,
        "version": "0.1.0",
        "tasks": {
            "dev": "deno run --watch src/main.ts",
            "start": "deno run src/main.ts"
        },
        "imports": imports
    });

    let deno_json = serde_json::to_string_pretty(&deno_json_val)
        .map_err(|e| format!("Failed to serialize deno.json: {e}"))?;

    send(tx, "Writing deno.json...");
    writer::write_file(base, "deno.json", &deno_json)?;

    send(tx, "Creating src/main.ts...");
    write_deno_entry(base, framework)
}

fn write_deno_entry(base: &Path, framework: &str) -> Result<(), String> {
    use std::fs;
    let src = base.join("src");
    fs::create_dir_all(&src).map_err(|e| format!("Failed to create src/: {e}"))?;

    let content = match framework {
        "Oak" => {
            r#"import { Application, Router } from "oak";

const router = new Router();
router.get("/", (ctx) => {
  ctx.response.body = "Hello World!";
});

const app = new Application();
app.use(router.routes());
app.use(router.allowedMethods());

await app.listen({ port: 3000 });
"#
        }
        "Hono" => {
            r#"import { Hono } from "hono";

const app = new Hono();
app.get("/", (c) => c.text("Hello World!"));

Deno.serve({ port: 3000 }, app.fetch);
"#
        }
        _ => {
            r#"console.log("Hello World!");
"#
        }
    };

    writer::write_file(&src, "main.ts", content)
}

fn scaffold_node_bun(
    params: &ScaffoldParams,
    base: &Path,
    framework: &str,
    pm: &str,
    eslint: &str,
    tx: &Sender<String>,
) -> Result<(), String> {
    let name = &params.project_name;

    let mut deps: Vec<&str> = Vec::new();
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
            deps.extend_from_slice(&["@nestjs/core", "@nestjs/common", "rxjs", "reflect-metadata"]);
            dev_deps.push("@nestjs/cli");
        }
        "Hono" => {
            deps.push("hono");
        }
        "Elysia" => {
            deps.push("elysia");
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
                "prettier",
                "eslint-plugin-prettier",
                "eslint-config-prettier",
            ]);
        }
        _ => {}
    }

    let mut pkg = serde_json::json!({
        "name": name,
        "version": "0.1.0",
        "type": "module",
        "scripts": {
            "dev": "tsx watch src/index.ts",
            "build": "tsc",
            "start": "node dist/index.js"
        },
        "dependencies": {},
        "devDependencies": {}
    });

    if let Some(deps_obj) = pkg.get_mut("dependencies").and_then(|d| d.as_object_mut()) {
        for d in deps {
            deps_obj.insert(d.to_string(), serde_json::Value::String("latest".to_string()));
        }
    }

    if let Some(dev_deps_obj) = pkg.get_mut("devDependencies").and_then(|d| d.as_object_mut()) {
        for d in dev_deps {
            dev_deps_obj.insert(d.to_string(), serde_json::Value::String("latest".to_string()));
        }
    }

    let package_json = serde_json::to_string_pretty(&pkg)
        .map_err(|e| format!("Failed to serialize package.json: {e}"))?;

    send(tx, "Writing package.json...");
    writer::write_file(base, "package.json", &package_json)?;

    writer::ensure_js_linting_scripts(base, eslint)?;

    send(tx, format!("Running {pm} install..."));
    let (prog, args): (&str, &[&str]) = match pm {
        "pnpm" => ("pnpm", &["install"]),
        "yarn" => ("yarn", &[]),
        "bun" => ("bun", &["install"]),
        _ => ("npm", &["install"]),
    };
    run_in(base, prog, args, tx)?;

    send(tx, "Writing config files...");
    writer::write_eslint_files(base, eslint, writer::EslintTarget::Backend)?;

    send(tx, "Creating src/index.ts...");
    write_entry_file(base, framework)
}

fn write_entry_file(base: &Path, framework: &str) -> Result<(), String> {
    use std::fs;
    let src = base.join("src");
    fs::create_dir_all(&src).map_err(|e| format!("Failed to create src/: {e}"))?;

    let content = match framework {
        "Express" => {
            r#"import express from "express";

const app = express();
const port = 3000;

app.get("/", (_req, res) => {
  res.send("Hello World!");
});

app.listen(port, () => {
  console.log(`Server running at http://localhost:${port}`);
});
"#
        }
        "Fastify" => {
            r#"import Fastify from "fastify";

const app = Fastify({ logger: true });

app.get("/", async () => {
  return { message: "Hello World!" };
});

await app.listen({ port: 3000 });
"#
        }
        "Hono" => {
            r#"import { Hono } from "hono";
import { serve } from "@hono/node-server";

const app = new Hono();

app.get("/", (c) => c.text("Hello World!"));

serve({ fetch: app.fetch, port: 3000 });
"#
        }
        "Elysia" => {
            r#"import { Elysia } from "elysia";

const app = new Elysia()
  .get("/", () => "Hello World!")
  .listen(3000);

console.log(`Server running at http://localhost:${app.server?.port}`);
"#
        }
        "NestJS" => {
            r#"import { NestFactory } from "@nestjs/core";
import { AppModule } from "./app.module.js";

const app = await NestFactory.create(AppModule);
await app.listen(3000);
"#
        }
        _ => {
            r#"console.log("Hello World!");
"#
        }
    };

    writer::write_file(&src, "index.ts", content)
}

fn send(tx: &Sender<String>, msg: impl Into<String>) {
    let _ = tx.send(msg.into());
}
