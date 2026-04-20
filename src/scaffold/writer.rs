use std::fs;
use std::path::Path;

use super::writer_constants::{
    ESLINT_CUSTOM_STRICT, ESLINT_RECOMMENDED, ESLINT_RECOMMENDED_PRETTIER, PRETTIERRC,
    TSCONFIG_CUSTOM_STRICT, TSCONFIG_DEFAULT,
};

pub fn write_eslint_files(base: &Path, eslint_choice: &str) -> Result<(), String> {
    write_tsconfig_for_eslint(base, eslint_choice)?;
    write_eslint_config_files(base, eslint_choice)
}

pub fn write_eslint_config_files(base: &Path, eslint_choice: &str) -> Result<(), String> {
    match eslint_choice {
        "Recommended" => {
            write_file(base, "eslint.config.js", ESLINT_RECOMMENDED)?;
        }
        "Recommended + Prettier" => {
            write_file(base, "eslint.config.js", ESLINT_RECOMMENDED_PRETTIER)?;
            write_file(base, ".prettierrc", PRETTIERRC)?;
        }
        "Custom Strict" => {
            write_file(base, "eslint.config.js", ESLINT_CUSTOM_STRICT)?;
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
