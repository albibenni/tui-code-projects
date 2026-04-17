use super::shared::git_hooks_general_step;
use super::types::{Category, Choice, Language, OptionStep};

pub fn rust_language() -> Language {
    Language {
        name: "Rust",
        category: Category::Backend,
        steps: vec![
            OptionStep {
                title: "Project Type",
                choices: vec![
                    Choice {
                        name: "Binary",
                        description: "Executable binary crate",
                        follow_up: vec![],
                    },
                    Choice {
                        name: "Library",
                        description: "Reusable library crate",
                        follow_up: vec![],
                    },
                    Choice {
                        name: "Web API",
                        description: "HTTP server application",
                        follow_up: vec![OptionStep {
                            title: "Framework",
                            choices: vec![
                                Choice {
                                    name: "Axum",
                                    description: "Modular web framework by Tokio",
                                    follow_up: vec![],
                                },
                                Choice {
                                    name: "Actix-web",
                                    description: "Powerful and fast web framework",
                                    follow_up: vec![],
                                },
                                Choice {
                                    name: "Rocket",
                                    description: "Web framework focused on ease of use",
                                    follow_up: vec![],
                                },
                                Choice {
                                    name: "Warp",
                                    description: "Composable web server framework",
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
