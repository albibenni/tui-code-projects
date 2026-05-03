use super::types::{Choice, OptionStep};

pub fn pm_js() -> OptionStep {
    let eslint = eslint_frontend_step();
    OptionStep::single(
        "Package Manager",
        vec![
            Choice {
                name: "npm",
                description: "Default Node.js package manager",
                follow_up: vec![eslint.clone()],
            },
            Choice {
                name: "pnpm",
                description: "Fast, disk-efficient package manager",
                follow_up: vec![eslint.clone()],
            },
            Choice {
                name: "bun",
                description: "Fast JavaScript all-in-one toolkit",
                follow_up: vec![eslint],
            },
        ],
    )
}

pub fn pm_python() -> OptionStep {
    OptionStep::single(
        "Package Manager",
        vec![
            Choice {
                name: "pip",
                description: "Standard Python package manager",
                follow_up: vec![],
            },
            Choice {
                name: "poetry",
                description: "Dependency management and packaging",
                follow_up: vec![],
            },
            Choice {
                name: "uv",
                description: "Fast Python package installer",
                follow_up: vec![],
            },
        ],
    )
}

pub fn eslint_backend_step() -> OptionStep {
    OptionStep::single(
        "ESLint",
        vec![
            Choice {
                name: "None",
                description: "Skip ESLint",
                follow_up: vec![],
            },
            Choice {
                name: "Recommended",
                description: "typescript-eslint recommended, flat config (eslint.config.js)",
                follow_up: vec![],
            },
            Choice {
                name: "Recommended + Prettier",
                description: "Recommended + eslint-plugin-prettier, flat config",
                follow_up: vec![],
            },
            Choice {
                name: "Custom Strict",
                description: "Full preset: eslint.config.js + tsconfig.json + .prettierrc",
                follow_up: vec![],
            },
        ],
    )
}

pub fn eslint_frontend_step() -> OptionStep {
    let hooks = git_hooks_ts_step();
    OptionStep::single(
        "ESLint",
        vec![
            Choice {
                name: "None",
                description: "Skip ESLint",
                follow_up: vec![hooks.clone()],
            },
            Choice {
                name: "Recommended",
                description: "typescript-eslint recommended, flat config (eslint.config.js)",
                follow_up: vec![hooks.clone()],
            },
            Choice {
                name: "Recommended + Prettier",
                description: "Recommended + eslint-plugin-prettier, flat config",
                follow_up: vec![hooks.clone()],
            },
            Choice {
                name: "Custom Strict",
                description: "Full preset: eslint.config.js + tsconfig.json + .prettierrc",
                follow_up: vec![hooks],
            },
        ],
    )
}

pub fn testing_step() -> OptionStep {
    OptionStep::single(
        "Testing",
        vec![
            Choice {
                name: "None",
                description: "No testing framework",
                follow_up: vec![],
            },
            Choice {
                name: "Vitest",
                description: "Fast, Vite-native testing framework",
                follow_up: vec![],
            },
        ],
    )
}

pub fn git_hooks_ts_step() -> OptionStep {
    let testing = testing_step();
    OptionStep::single(
        "Git Hooks",
        vec![
            Choice {
                name: "None",
                description: "Do not configure pre-commit hooks",
                follow_up: vec![testing.clone()],
            },
            Choice {
                name: "Husky (lint + test)",
                description: "Use Husky pre-commit to run lint and test",
                follow_up: vec![testing],
            },
        ],
    )
}

pub fn git_hooks_general_step() -> OptionStep {
    OptionStep::single(
        "Git Hooks",
        vec![
            Choice {
                name: "None",
                description: "Do not configure pre-commit hooks",
                follow_up: vec![],
            },
            Choice {
                name: "Native Git Hook (make lint && make test)",
                description: "Use .git/hooks/pre-commit with Makefile targets",
                follow_up: vec![],
            },
            Choice {
                name: "Lefthook (lint + test)",
                description: "Use Lefthook pre-commit config for lint and test",
                follow_up: vec![],
            },
        ],
    )
}
