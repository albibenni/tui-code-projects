use std::fs;
use std::path::Path;
use std::sync::mpsc::Sender;

use super::params::ScaffoldParams;
use super::writer::write_file;

pub fn scaffold(params: &ScaffoldParams, base: &Path, tx: &Sender<String>) -> Result<(), String> {
    let project_type = params.sel("Project Type").unwrap_or("CLI");
    let framework = params.sel("Framework");
    let dep_manager = params.sel("Dependency Manager").unwrap_or("Composer");

    let _ = tx.send("Writing PHP project files...".to_string());

    write_file(
        base,
        "README.md",
        &readme(project_type, framework, dep_manager),
    )?;

    if dep_manager == "Composer" {
        write_file(base, "composer.json", &composer_json(params))?;
    }

    match project_type {
        "Web API" => {
            fs::create_dir_all(base.join("public"))
                .map_err(|e| format!("Failed to create public/: {e}"))?;
            write_file(base, "public/index.php", &web_entry(framework))?;
        }
        _ => {
            write_file(base, "index.php", &cli_entry())?;
        }
    }

    write_file(
        base,
        "Makefile",
        makefile(project_type, dep_manager == "Composer"),
    )?;

    Ok(())
}

fn readme(project_type: &str, framework: Option<&str>, dep_manager: &str) -> String {
    match (project_type, framework) {
        ("Web API", Some("Laravel")) => format!(
            "# PHP API\n\n- Framework: Laravel\n- Dependency Manager: {dep_manager}\n\nRun with your preferred PHP toolchain.\n"
        ),
        ("Web API", Some("Symfony")) => format!(
            "# PHP API\n\n- Framework: Symfony\n- Dependency Manager: {dep_manager}\n\nRun with your preferred PHP toolchain.\n"
        ),
        ("Web API", Some("Slim")) => format!(
            "# PHP API\n\n- Framework: Slim\n- Dependency Manager: {dep_manager}\n\nRun with your preferred PHP toolchain.\n"
        ),
        _ => format!(
            "# PHP CLI\n\n- Dependency Manager: {dep_manager}\n\nRun with your preferred PHP toolchain.\n"
        ),
    }
}

fn composer_json(params: &ScaffoldParams) -> String {
    format!(
        r#"{{
  "name": "example/{name}",
  "type": "project",
  "require": {{}}
}}
"#,
        name = params.project_name
    )
}

fn cli_entry() -> String {
    "<?php\n\necho \"Hello World!\\n\";\n".to_string()
}

fn web_entry(framework: Option<&str>) -> String {
    match framework {
        Some("Slim") => r#"<?php

require __DIR__ . '/../vendor/autoload.php';

$app = \Slim\Factory\AppFactory::create();
$app->get('/', function ($request, $response) {
    $response->getBody()->write('Hello World!');
    return $response;
});
$app->run();
"#
        .to_string(),
        Some("Laravel") => r#"<?php

// This scaffold creates a minimal entrypoint placeholder.
// Typical Laravel setup: composer create-project laravel/laravel .
echo "Laravel placeholder";
"#
        .to_string(),
        Some("Symfony") => r#"<?php

// This scaffold creates a minimal entrypoint placeholder.
// Typical Symfony setup: composer create-project symfony/skeleton .
echo "Symfony placeholder";
"#
        .to_string(),
        _ => "<?php\n\necho \"Hello World!\";\n".to_string(),
    }
}

fn makefile(project_type: &str, use_composer: bool) -> &'static str {
    match (project_type, use_composer) {
        ("Web API", true) => {
            r#"PHP ?= php
COMPOSER ?= composer

.PHONY: install run test lint

install:
	@$(COMPOSER) install

run:
	@$(PHP) -S localhost:8000 -t public

test:
	@$(PHP) -v

lint:
	@find . -name "*.php" -print0 | xargs -0 -n1 $(PHP) -l
"#
        }
        ("Web API", false) => {
            r#"PHP ?= php

.PHONY: run test lint

run:
	@$(PHP) -S localhost:8000 -t public

test:
	@$(PHP) -v

lint:
	@find . -name "*.php" -print0 | xargs -0 -n1 $(PHP) -l
"#
        }
        (_, true) => {
            r#"PHP ?= php
COMPOSER ?= composer

.PHONY: install run test lint

install:
	@$(COMPOSER) install

run:
	@$(PHP) index.php

test:
	@$(PHP) -v

lint:
	@find . -name "*.php" -print0 | xargs -0 -n1 $(PHP) -l
"#
        }
        _ => {
            r#"PHP ?= php

.PHONY: run test lint

run:
	@$(PHP) index.php

test:
	@$(PHP) -v

lint:
	@find . -name "*.php" -print0 | xargs -0 -n1 $(PHP) -l
"#
        }
    }
}
