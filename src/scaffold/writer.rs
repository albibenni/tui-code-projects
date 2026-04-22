use std::fs;
use std::path::Path;

use super::writer_constants::{
    BACKEND_ESLINT_CUSTOM_STRICT, BACKEND_ESLINT_RECOMMENDED, BACKEND_ESLINT_RECOMMENDED_PRETTIER,
    FRONTEND_ESLINT_CUSTOM_STRICT, FRONTEND_ESLINT_RECOMMENDED,
    FRONTEND_ESLINT_RECOMMENDED_PRETTIER, PRETTIERRC, TSCONFIG_CUSTOM_STRICT, TSCONFIG_DEFAULT,
};

#[derive(Clone, Copy)]
pub enum EslintTarget {
    Frontend,
    Backend,
}

pub fn write_eslint_files(base: &Path, eslint_choice: &str, target: EslintTarget) -> Result<(), String> {
    write_tsconfig_for_eslint(base, eslint_choice)?;
    write_eslint_config_files(base, eslint_choice, target)
}

pub fn ensure_js_linting_scripts(base: &Path, eslint_choice: &str) -> Result<(), String> {
    let scripts: &[(&str, &str)] = match eslint_choice {
        "Recommended" => &[("lint", "eslint .")],
        "Recommended + Prettier" | "Custom Strict" => &[
            ("lint", "eslint ."),
            ("format", "prettier . --write"),
            ("format:check", "prettier . --check"),
        ],
        _ => &[],
    };

    if scripts.is_empty() {
        return Ok(());
    }

    ensure_package_json_scripts(base, scripts)
}

pub fn write_eslint_config_files(
    base: &Path,
    eslint_choice: &str,
    target: EslintTarget,
) -> Result<(), String> {
    let (recommended, recommended_prettier, custom_strict) = match target {
        EslintTarget::Frontend => (
            FRONTEND_ESLINT_RECOMMENDED,
            FRONTEND_ESLINT_RECOMMENDED_PRETTIER,
            FRONTEND_ESLINT_CUSTOM_STRICT,
        ),
        EslintTarget::Backend => (
            BACKEND_ESLINT_RECOMMENDED,
            BACKEND_ESLINT_RECOMMENDED_PRETTIER,
            BACKEND_ESLINT_CUSTOM_STRICT,
        ),
    };

    match eslint_choice {
        "Recommended" => {
            write_file(base, "eslint.config.js", recommended)?;
        }
        "Recommended + Prettier" => {
            write_file(base, "eslint.config.js", recommended_prettier)?;
            write_file(base, ".prettierrc", PRETTIERRC)?;
        }
        "Custom Strict" => {
            write_file(base, "eslint.config.js", custom_strict)?;
            write_file(base, ".prettierrc", PRETTIERRC)?;
        }
        _ => {}
    }
    Ok(())
}

fn write_tsconfig_for_eslint(base: &Path, eslint_choice: &str) -> Result<(), String> {
    match eslint_choice {
        "Custom Strict" => write_file(base, "tsconfig.json", TSCONFIG_CUSTOM_STRICT),
        _ => write_file(base, "tsconfig.json", TSCONFIG_DEFAULT),
    }
}

pub fn write_file(base: &Path, name: &str, content: &str) -> Result<(), String> {
    fs::write(base.join(name), content).map_err(|e| format!("Failed to write {name}: {e}"))
}

fn ensure_package_json_scripts(base: &Path, scripts: &[(&str, &str)]) -> Result<(), String> {
    let package_path = base.join("package.json");
    let content = fs::read_to_string(&package_path)
        .map_err(|e| format!("Failed to read package.json for script update: {e}"))?;

    let mut pkg: serde_json::Value = serde_json::from_str(&content)
        .map_err(|e| format!("Failed to parse package.json: {e}"))?;

    if let Some(scripts_obj) = pkg.get_mut("scripts").and_then(|s| s.as_object_mut()) {
        for (name, cmd) in scripts {
            if !scripts_obj.contains_key(*name) {
                scripts_obj.insert(name.to_string(), serde_json::Value::String(cmd.to_string()));
            }
        }
    } else {
        // Create scripts object if it doesn't exist
        let mut scripts_obj = serde_json::Map::new();
        for (name, cmd) in scripts {
            scripts_obj.insert(name.to_string(), serde_json::Value::String(cmd.to_string()));
        }
        pkg.as_object_mut().unwrap().insert("scripts".to_string(), serde_json::Value::Object(scripts_obj));
    }

    let updated_content = serde_json::to_string_pretty(&pkg)
        .map_err(|e| format!("Failed to serialize package.json: {e}"))?;
    
    fs::write(package_path, updated_content)
        .map_err(|e| format!("Failed to update package.json scripts: {e}"))
}

// Remove the old manual parsing function as it's no longer needed
