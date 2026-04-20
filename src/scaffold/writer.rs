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
