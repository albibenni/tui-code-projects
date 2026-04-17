use super::shared::git_hooks_general_step;
use super::types::{Category, Choice, Language, OptionStep};

pub fn go_desktop_language() -> Language {
    Language {
        name: "Go (Desktop)",
        category: Category::Desktop,
        steps: vec![
            OptionStep {
                title: "Framework",
                choices: vec![
                    Choice {
                        name: "Fyne",
                        description: "Cross-platform GUI toolkit written in Go",
                        follow_up: vec![],
                    },
                    Choice {
                        name: "Gio",
                        description: "Immediate mode GUI for Go, hardware accelerated",
                        follow_up: vec![],
                    },
                ],
            },
            git_hooks_general_step(),
        ],
    }
}
