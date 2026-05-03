pub const TSCONFIG_DEFAULT: &str = r#"{
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

pub const TSCONFIG_CUSTOM_STRICT: &str = r#"{
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

pub const BACKEND_ESLINT_RECOMMENDED: &str = r#"// @ts-check
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

pub const BACKEND_ESLINT_RECOMMENDED_PRETTIER: &str = r#"// @ts-check
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

pub const BACKEND_ESLINT_CUSTOM_STRICT: &str = r#"// @ts-check
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
      },
      ecmaVersion: "latest",
      sourceType: "module",
      parserOptions: {
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

pub const FRONTEND_ESLINT_RECOMMENDED: &str = r#"// @ts-check
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
        ...globals.browser,
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

pub const FRONTEND_ESLINT_RECOMMENDED_PRETTIER: &str = r#"// @ts-check
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
        ...globals.browser,
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

pub const FRONTEND_ESLINT_CUSTOM_STRICT: &str = r#"// @ts-check
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
        ...globals.browser,
        ...globals.node,
        ...globals.vitest,
      },
      ecmaVersion: "latest",
      sourceType: "module",
      parserOptions: {
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

pub const PRETTIERRC: &str = r#"{
  "trailingComma": "all",
  "tabWidth": 2,
  "semi": true,
  "singleQuote": false
}
"#;

pub const VITEST_BACKEND_CONFIG: &str = r#"/// <reference types="vitest" />
import { defineConfig } from "vitest/config";

export default defineConfig({
  test: {
    globals: false,
    environment: "node",
    coverage: {
      provider: "v8",
      reporter: ["text", "json", "html"],
    },
  },
});
"#;

pub const VITEST_FRONTEND_CONFIG: &str = r#"/// <reference types="vitest" />
import { defineConfig } from "vitest/config";

export default defineConfig({
  test: {
    globals: false,
    environment: "happy-dom",
    coverage: {
      provider: "v8",
      reporter: ["text", "json", "html"],
    },
  },
});
"#;

pub const VITEST_SAMPLE_TEST: &str = r#"import { describe, it, expect } from "vitest";

describe("Sample Test", () => {
  it("should add 1 + 1 correctly", () => {
    expect(1 + 1).toBe(2);
  });
});
"#;
