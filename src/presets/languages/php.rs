use super::shared::git_hooks_general_step;
use super::types::{Category, Choice, Language, OptionStep};

pub fn php_language() -> Language {
    Language {
        name: "PHP",
        category: Category::Backend,
        steps: vec![
            OptionStep::single(
                "Project Type",
                vec![
                    Choice {
                        name: "CLI",
                        description: "Command-line script or tool",
                        follow_up: vec![deps_step()],
                    },
                    Choice {
                        name: "Web API",
                        description: "HTTP server application",
                        follow_up: vec![OptionStep::single(
                            "Framework",
                            vec![
                                Choice {
                                    name: "Laravel",
                                    description: "Full-featured modern PHP framework",
                                    follow_up: vec![deps_step()],
                                },
                                Choice {
                                    name: "Symfony",
                                    description: "Reusable components and framework",
                                    follow_up: vec![deps_step()],
                                },
                                Choice {
                                    name: "Slim",
                                    description: "Micro-framework for APIs",
                                    follow_up: vec![deps_step()],
                                },
                            ],
                        )],
                    },
                ],
            ),
            git_hooks_general_step(),
        ],
    }
}

fn deps_step() -> OptionStep {
    OptionStep::single(
        "Dependency Manager",
        vec![
            Choice {
                name: "Composer",
                description: "Standard PHP dependency manager",
                follow_up: vec![],
            },
            Choice {
                name: "None",
                description: "No dependency manager setup",
                follow_up: vec![],
            },
        ],
    )
}
