use super::types::{Category, Choice, Language, OptionStep};

pub fn swift_language() -> Language {
    Language {
        name: "Swift",
        category: Category::Desktop,
        steps: vec![OptionStep {
            title: "UI Framework",
            choices: vec![
                Choice {
                    name: "SwiftUI",
                    description: "Declarative UI framework for Apple platforms",
                    follow_up: vec![OptionStep {
                        title: "Core Language",
                        choices: vec![
                            Choice {
                                name: "Swift Only",
                                description: "Pure Swift application",
                                follow_up: vec![],
                            },
                            Choice {
                                name: "Swift + Rust",
                                description: "SwiftUI frontend with Rust core via swift-bridge",
                                follow_up: vec![],
                            },
                        ],
                    }],
                },
                Choice {
                    name: "AppKit",
                    description: "Traditional macOS UI framework",
                    follow_up: vec![],
                },
            ],
        }],
    }
}
