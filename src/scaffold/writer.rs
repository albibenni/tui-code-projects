use std::fs;
use std::path::Path;

const TSCONFIG_DEFAULT: &str = r#"{
  "compilerOptions": {
    "target": "ESNext",
    "module": "NodeNext",
    "moduleResolution": "NodeNext",
    "strict": true,
    "esModuleInterop": true,
    "skipLibCheck": true,
    "outDir": "dist",
    "sourceMap": true
  },
  "include": ["src"],
  "exclude": ["node_modules", "dist"]
}
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

const PRETTIERRC: &str = r#"{
  "trailingComma": "all",
  "tabWidth": 2,
  "semi": true,
  "singleQuote": false
}
"#;

pub fn write_eslint_files(base: &Path, eslint_choice: &str) -> Result<(), String> {
    match eslint_choice {
        "Recommended" => {
            write_file(base, "tsconfig.json", TSCONFIG_DEFAULT)?;
            write_file(base, "eslint.config.js", ESLINT_RECOMMENDED)?;
        }
        "Recommended + Prettier" => {
            write_file(base, "tsconfig.json", TSCONFIG_DEFAULT)?;
            write_file(base, "eslint.config.js", ESLINT_RECOMMENDED_PRETTIER)?;
            write_file(base, ".prettierrc", PRETTIERRC)?;
        }
        "Custom Strict" => {
            write_file(base, "tsconfig.json", TSCONFIG_CUSTOM_STRICT)?;
            write_file(base, "eslint.config.js", ESLINT_CUSTOM_STRICT)?;
            write_file(base, ".prettierrc", PRETTIERRC)?;
        }
        _ => {
            write_file(base, "tsconfig.json", TSCONFIG_DEFAULT)?;
        }
    }
    Ok(())
}

pub fn write_file(base: &Path, name: &str, content: &str) -> Result<(), String> {
    fs::write(base.join(name), content).map_err(|e| format!("Failed to write {name}: {e}"))
}
