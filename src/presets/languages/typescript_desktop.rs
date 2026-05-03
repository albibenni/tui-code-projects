use super::shared::pm_js;
use super::types::{Category, Choice, Language, OptionStep};

pub fn typescript_desktop_language() -> Language {
    let pm = pm_js();

    Language {
        name: "TypeScript (Desktop)",
        category: Category::Desktop,
        steps: vec![OptionStep::single(
            "Framework",
            vec![
                Choice {
                    name: "Tauri",
                    description: "Lightweight, secure, and cross-platform (Rust backend)",
                    follow_up: vec![pm.clone()],
                },
                Choice {
                    name: "Electron",
                    description: "Build cross-platform desktop apps with Web Technologies",
                    follow_up: vec![pm.clone()],
                },
                Choice {
                    name: "NeutralinoJS",
                    description: "Lightweight cross-platform desktop application framework",
                    follow_up: vec![pm],
                },
            ],
        )],
    }
}
