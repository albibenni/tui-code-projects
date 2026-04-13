use std::fs;
use std::path::PathBuf;

use crate::app::App;

const ESLINT_RECOMMENDED: &str = r#"// @ts-check
import eslint from "@eslint/js";
import globals from "globals";
import { defineConfig } from "eslint/config";
import { configs } from "typescript-eslint";

export default defineConfig(
  eslint.configs.recommended,
  ...configs.recommendedTypeChecked,
  {
    languageOptions: {
      globals: {
        ...globals.es2025,
        ...globals.node,
      },
      ecmaVersion: "latest",
      sourceType: "module",
      parserOptions: {
        projectService: true,
        tsconfigRootDir: import.meta.dirname,
      },
    },
  },
);
"#;

const ESLINT_RECOMMENDED_PRETTIER: &str = r#"// @ts-check
import eslint from "@eslint/js";
import eslintPluginPrettierRecommended from "eslint-plugin-prettier/recommended";
import globals from "globals";
import { defineConfig } from "eslint/config";
import { configs } from "typescript-eslint";

export default defineConfig(
  eslint.configs.recommended,
  ...configs.recommendedTypeChecked,
  eslintPluginPrettierRecommended,
  {
    languageOptions: {
      globals: {
        ...globals.es2025,
        ...globals.node,
      },
      ecmaVersion: "latest",
      sourceType: "module",
      parserOptions: {
        projectService: true,
        tsconfigRootDir: import.meta.dirname,
      },
    },
  },
);
"#;

const ESLINT_CUSTOM_STRICT: &str = r#"// @ts-check
import eslint from "@eslint/js";
import eslintPluginPrettierRecommended from "eslint-plugin-prettier/recommended";
import globals from "globals";
import { defineConfig } from "eslint/config";
import { configs } from "typescript-eslint";

export default defineConfig(
  { ignores: ["eslint.config.js"] },
  eslint.configs.recommended,
  ...configs.recommendedTypeChecked,
  eslintPluginPrettierRecommended,
  {
    languageOptions: {
      globals: {
        ...globals.es2025,
        ...globals.node,
        ...globals.vitest,
        React: "readonly",
      },
      ecmaVersion: "latest",
      sourceType: "module",
      parserOptions: {
        ecmaFeatures: { jsx: true },
        projectService: {
          allowDefaultProject: ["*.js", "*.cjs", "eslint.config.js"],
        },
        tsconfigRootDir: import.meta.dirname,
      },
    },
  },
  {
    rules: {
      "@typescript-eslint/explicit-module-boundary-types": "off",
      "@typescript-eslint/no-inferrable-types": "off",
      "@typescript-eslint/no-non-null-assertion": "off",
      "@typescript-eslint/no-empty-interface": "off",
      "@typescript-eslint/no-namespace": "off",
      "@typescript-eslint/no-empty-function": "off",
      "@typescript-eslint/no-this-alias": "off",
      "@typescript-eslint/ban-types": "off",
      "@typescript-eslint/no-unsafe-declaration-merging": "error",
      "@typescript-eslint/ban-ts-comment": "off",
      "prefer-spread": "off",
      "no-case-declarations": "off",
      "no-console": "off",
      "@typescript-eslint/no-unused-vars": ["warn"],
      "@typescript-eslint/consistent-type-imports": "warn",
      "@typescript-eslint/no-unnecessary-condition": "warn",
      "@typescript-eslint/no-explicit-any": "off",
      "@typescript-eslint/no-floating-promises": "warn",
      "@typescript-eslint/no-unsafe-argument": "warn",
    },
  },
);
"#;

const TSCONFIG_CUSTOM_STRICT: &str = r#"{
  "compilerOptions": {
    "target": "ESNext",
    "module": "NodeNext",
    "moduleResolution": "NodeNext",
    "moduleDetection": "force",
    "verbatimModuleSyntax": true,
    "resolveJsonModule": true,
    "esModuleInterop": true,
    "noEmit": true,
    "allowImportingTsExtensions": true,
    "erasableSyntaxOnly": true,
    "allowJs": true,
    "types": ["node"],
    "noImplicitOverride": true,
    "noUncheckedIndexedAccess": true,
    "strict": true,
    "forceConsistentCasingInFileNames": true,
    "skipLibCheck": true,
    "sourceMap": true
  }
}
"#;

const PRETTIERRC_CUSTOM_STRICT: &str = r#"{
  "trailingComma": "all",
  "tabWidth": 2,
  "semi": true,
  "singleQuote": false
}
"#;

pub fn run(app: &App) -> Result<String, String> {
    let base: PathBuf = [&app.config.project_path, &app.config.project_name]
        .iter()
        .collect();

    fs::create_dir_all(&base).map_err(|e| format!("Failed to create directory: {e}"))?;

    let eslint_choice = app
        .option_selections
        .iter()
        .find(|s| s.title == "ESLint")
        .map(|s| s.choice_name);

    match eslint_choice {
        Some("Recommended") => {
            write_file(&base, "eslint.config.js", ESLINT_RECOMMENDED)?;
        }
        Some("Recommended + Prettier") => {
            write_file(&base, "eslint.config.js", ESLINT_RECOMMENDED_PRETTIER)?;
        }
        Some("Custom Strict") => {
            write_file(&base, "eslint.config.js", ESLINT_CUSTOM_STRICT)?;
            write_file(&base, "tsconfig.json", TSCONFIG_CUSTOM_STRICT)?;
            write_file(&base, ".prettierrc", PRETTIERRC_CUSTOM_STRICT)?;
        }
        _ => {}
    }

    Ok(base.display().to_string())
}

fn write_file(base: &PathBuf, name: &str, content: &str) -> Result<(), String> {
    fs::write(base.join(name), content).map_err(|e| format!("Failed to write {name}: {e}"))
}
