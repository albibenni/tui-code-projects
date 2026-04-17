use super::types::{Category, Choice, Language, OptionStep};

pub fn java_language() -> Language {
    Language {
        name: "Java",
        category: Category::Backend,
        steps: vec![OptionStep {
            title: "Project Type",
            choices: vec![
                Choice {
                    name: "CLI",
                    description: "Command-line application",
                    follow_up: vec![build_tool_step()],
                },
                Choice {
                    name: "Web API",
                    description: "HTTP server application",
                    follow_up: vec![OptionStep {
                        title: "Framework",
                        choices: vec![
                            Choice {
                                name: "Spring Boot",
                                description: "Enterprise-ready Java framework",
                                follow_up: vec![build_tool_step()],
                            },
                            Choice {
                                name: "Micronaut",
                                description: "Lightweight JVM microservice framework",
                                follow_up: vec![build_tool_step()],
                            },
                            Choice {
                                name: "Javalin",
                                description: "Simple, lightweight web framework",
                                follow_up: vec![build_tool_step()],
                            },
                        ],
                    }],
                },
            ],
        }],
    }
}

fn build_tool_step() -> OptionStep {
    OptionStep {
        title: "Build Tool",
        choices: vec![
            Choice {
                name: "Maven",
                description: "Standard Java build automation tool",
                follow_up: vec![],
            },
            Choice {
                name: "Gradle",
                description: "Flexible build tool with Kotlin/Groovy DSL",
                follow_up: vec![],
            },
        ],
    }
}
