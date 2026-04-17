use super::shared::pm_python;
use super::types::{Category, Choice, Language, OptionStep};

pub fn python_language() -> Language {
    Language {
        name: "Python",
        category: Category::Backend,
        steps: vec![OptionStep {
            title: "Project Type",
            choices: vec![
                Choice {
                    name: "Script",
                    description: "Simple standalone script",
                    follow_up: vec![pm_python()],
                },
                Choice {
                    name: "CLI",
                    description: "Command-line tool with argparse",
                    follow_up: vec![pm_python()],
                },
                Choice {
                    name: "Web API",
                    description: "HTTP server application",
                    follow_up: vec![OptionStep {
                        title: "Framework",
                        choices: vec![
                            Choice {
                                name: "FastAPI",
                                description: "Modern async REST API framework",
                                follow_up: vec![pm_python()],
                            },
                            Choice {
                                name: "Flask",
                                description: "Lightweight WSGI web framework",
                                follow_up: vec![pm_python()],
                            },
                            Choice {
                                name: "Django",
                                description: "Batteries-included web framework",
                                follow_up: vec![pm_python()],
                            },
                        ],
                    }],
                },
                Choice {
                    name: "Data Science",
                    description: "Data analysis and ML project",
                    follow_up: vec![OptionStep {
                        title: "Package Manager",
                        choices: vec![
                            Choice {
                                name: "pip",
                                description: "Standard Python package manager",
                                follow_up: vec![],
                            },
                            Choice {
                                name: "conda",
                                description: "Cross-platform package manager",
                                follow_up: vec![],
                            },
                            Choice {
                                name: "poetry",
                                description: "Dependency management and packaging",
                                follow_up: vec![],
                            },
                            Choice {
                                name: "uv",
                                description: "Fast Python package installer",
                                follow_up: vec![],
                            },
                        ],
                    }],
                },
            ],
        }],
    }
}
