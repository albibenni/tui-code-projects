use super::shared::git_hooks_general_step;
use super::types::{Category, Choice, Language, OptionStep};

pub fn flutter_language() -> Language {
    Language {
        name: "Flutter",
        category: Category::Mobile,
        steps: vec![
            OptionStep {
                title: "Start Configuration",
                choices: vec![
                    Choice {
                        name: "Mobile (Android + iOS)",
                        description: "Optimized starter setup for phone apps",
                        follow_up: vec![],
                    },
                    Choice {
                        name: "Web",
                        description: "Single-page Flutter web app setup",
                        follow_up: vec![],
                    },
                    Choice {
                        name: "Desktop",
                        description: "Desktop targets (macOS, Windows, Linux)",
                        follow_up: vec![],
                    },
                    Choice {
                        name: "All Platforms",
                        description: "Generate once for mobile, web, and desktop",
                        follow_up: vec![],
                    },
                ],
            },
            OptionStep {
                title: "State Management",
                choices: vec![
                    Choice {
                        name: "Provider",
                        description: "Simple and lightweight for most apps",
                        follow_up: vec![],
                    },
                    Choice {
                        name: "Riverpod",
                        description: "Type-safe and scalable architecture",
                        follow_up: vec![],
                    },
                    Choice {
                        name: "BLoC",
                        description: "Event-driven architecture for complex flows",
                        follow_up: vec![],
                    },
                    Choice {
                        name: "None (Vanilla)",
                        description: "Start minimal and choose later",
                        follow_up: vec![],
                    },
                ],
            },
            git_hooks_general_step(),
        ],
    }
}
