use super::shared::git_hooks_general_step;
use super::types::{Category, Choice, Language, OptionStep};

pub fn go_language() -> Language {
    Language {
        name: "Go",
        category: Category::Backend,
        steps: vec![
            OptionStep {
                title: "Project Type",
                choices: vec![
                    Choice {
                        name: "CLI",
                        description: "Command-line application",
                        follow_up: vec![],
                    },
                    Choice {
                        name: "Library",
                        description: "Reusable Go package",
                        follow_up: vec![],
                    },
                    Choice {
                        name: "Web API",
                        description: "HTTP server application",
                        follow_up: vec![OptionStep {
                            title: "Framework",
                            choices: vec![
                                Choice {
                                    name: "Gin",
                                    description: "Fast HTTP web framework",
                                    follow_up: vec![],
                                },
                                Choice {
                                    name: "Echo",
                                    description: "High performance web framework",
                                    follow_up: vec![],
                                },
                                Choice {
                                    name: "Fiber",
                                    description: "Express-inspired web framework",
                                    follow_up: vec![],
                                },
                                Choice {
                                    name: "Chi",
                                    description: "Lightweight and composable router",
                                    follow_up: vec![],
                                },
                                Choice {
                                    name: "net/http",
                                    description: "Standard library HTTP",
                                    follow_up: vec![],
                                },
                            ],
                        }],
                    },
                ],
            },
            git_hooks_general_step(),
        ],
    }
}
