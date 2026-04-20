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
    let mut content = fs::read_to_string(&package_path)
        .map_err(|e| format!("Failed to read package.json for script update: {e}"))?;

    let (start, end) = scripts_object_range(&content)
        .ok_or_else(|| "Failed to locate scripts block in package.json".to_string())?;
    let scripts_body = &content[start..end];

    let missing_entries: Vec<String> = scripts
        .iter()
        .filter(|(name, _)| !scripts_body.contains(&format!("\"{name}\"")))
        .map(|(name, cmd)| format!("    \"{name}\": \"{cmd}\""))
        .collect();

    if missing_entries.is_empty() {
        return Ok(());
    }

    let trimmed = scripts_body.trim();
    let updated_body = if trimmed.is_empty() {
        format!("\n{}\n  ", missing_entries.join(",\n"))
    } else {
        format!("{scripts_body},\n{}", missing_entries.join(",\n"))
    };

    content.replace_range(start..end, &updated_body);
    fs::write(package_path, content).map_err(|e| format!("Failed to update package.json scripts: {e}"))
}

fn scripts_object_range(content: &str) -> Option<(usize, usize)> {
    let scripts_key = content.find("\"scripts\"")?;
    let object_start = content[scripts_key..].find('{')? + scripts_key + 1;
    let mut depth = 1usize;
    let mut in_string = false;
    let mut escaped = false;

    for (idx, ch) in content[object_start..].char_indices() {
        if in_string {
            if escaped {
                escaped = false;
                continue;
            }
            match ch {
                '\\' => escaped = true,
                '"' => in_string = false,
                _ => {}
            }
            continue;
        }

        match ch {
            '"' => in_string = true,
            '{' => depth += 1,
            '}' => {
                depth -= 1;
                if depth == 0 {
                    let end = object_start + idx;
                    return Some((object_start, end));
                }
            }
            _ => {}
        }
    }
    None
}
