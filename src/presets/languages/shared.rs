use super::types::{Choice, OptionStep};

pub fn pm_js() -> OptionStep {
    OptionStep {
        title: "Package Manager",
        choices: vec![
            Choice { name: "npm",  description: "Default Node.js package manager",      follow_up: vec![] },
            Choice { name: "pnpm", description: "Fast, disk-efficient package manager", follow_up: vec![] },
            Choice { name: "bun",  description: "Fast JavaScript all-in-one toolkit",   follow_up: vec![] },
        ],
    }
}

pub fn pm_python() -> OptionStep {
    OptionStep {
        title: "Package Manager",
        choices: vec![
            Choice { name: "pip",    description: "Standard Python package manager",     follow_up: vec![] },
            Choice { name: "poetry", description: "Dependency management and packaging", follow_up: vec![] },
            Choice { name: "uv",     description: "Fast Python package installer",       follow_up: vec![] },
        ],
    }
}

pub fn eslint_backend_step() -> OptionStep {
    OptionStep {
        title: "ESLint",
        choices: vec![
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
    }
}
