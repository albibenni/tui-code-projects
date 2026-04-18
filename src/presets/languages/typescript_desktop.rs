use super::shared::{git_hooks_general_step, package_manager_step};
use super::types::{Category, Choice, Language, OptionStep};

pub fn typescript_desktop_language() -> Language {
    Language {
        name: "TypeScript (Desktop)",
        category: Category::Desktop,
        steps: vec![
            OptionStep {
                title: "Framework",
                choices: vec![
                    Choice {
                        name: "Electron",
                        description: "Build cross-platform desktop apps with Web Technologies",
                        follow_up: vec![],
                    },
                    Choice {
                        name: "NeutralinoJS",
                        description: "Lightweight cross-platform desktop application framework",
                        follow_up: vec![],
                    },
                ],
            },
            package_manager_step(),
            git_hooks_general_step(),
        ],
    }
}
