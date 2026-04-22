use super::shared::git_hooks_general_step;
use super::types::{Category, Choice, Language, OptionStep};

pub fn kotlin_mobile_language() -> Language {
    Language {
        name: "Kotlin (Mobile)",
        category: Category::Mobile,
        steps: vec![
            OptionStep::single(
                "UI Toolkit",
                vec![
                    Choice {
                        name: "Jetpack Compose",
                        description: "Modern declarative Android UI toolkit",
                        follow_up: vec![build_tool_step()],
                    },
                    Choice {
                        name: "XML Views",
                        description: "Traditional Android view system",
                        follow_up: vec![build_tool_step()],
                    },
                ],
            ),
            git_hooks_general_step(),
        ],
    }
}

fn build_tool_step() -> OptionStep {
    OptionStep::single(
        "Build Tool",
        vec![
            Choice {
                name: "Gradle (KTS)",
                description: "Gradle with Kotlin DSL scripts",
                follow_up: vec![],
            },
            Choice {
                name: "Gradle (Groovy)",
                description: "Gradle with Groovy scripts",
                follow_up: vec![],
            },
        ],
    )
}
