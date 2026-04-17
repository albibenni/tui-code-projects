use super::types::{Category, Choice, Language, OptionStep};

pub fn php_language() -> Language {
    Language {
        name: "PHP",
        category: Category::Backend,
        steps: vec![OptionStep {
            title: "Project Type",
            choices: vec![
                Choice {
                    name: "CLI",
                    description: "Command-line script or tool",
                    follow_up: vec![deps_step()],
                },
                Choice {
                    name: "Web API",
                    description: "HTTP server application",
                    follow_up: vec![OptionStep {
                        title: "Framework",
                        choices: vec![
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
                    }],
                },
            ],
        }],
    }
}

fn deps_step() -> OptionStep {
    OptionStep {
        title: "Dependency Manager",
        choices: vec![
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
    }
}
