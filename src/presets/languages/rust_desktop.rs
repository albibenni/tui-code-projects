use super::shared::git_hooks_general_step;
use super::types::{Category, Choice, Language, OptionStep};

pub fn rust_desktop_language() -> Language {
    Language {
        name: "Rust (Desktop)",
        category: Category::Desktop,
        steps: vec![
            OptionStep {
                title: "Framework",
                choices: vec![
                    Choice {
                        name: "Tauri",
                        description: "Cross-platform app with web frontend and Rust backend",
                        follow_up: vec![],
                    },
                    Choice {
                        name: "gtk4-rs",
                        description: "Native GTK4 bindings, primarily for Linux",
                        follow_up: vec![],
                    },
                ],
            },
            git_hooks_general_step(),
        ],
    }
}
