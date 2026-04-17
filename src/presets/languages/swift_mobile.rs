use super::types::{Category, Choice, Language, OptionStep};

pub fn swift_mobile_language() -> Language {
    Language {
        name: "Swift (Mobile)",
        category: Category::Mobile,
        steps: vec![OptionStep {
            title: "Target",
            choices: vec![Choice {
                name: "iOS App",
                description: "Native iPhone/iPad application",
                follow_up: vec![ui_framework_step()],
            }],
        }],
    }
}

fn ui_framework_step() -> OptionStep {
    OptionStep {
        title: "UI Framework",
        choices: vec![
            Choice {
                name: "SwiftUI",
                description: "Modern declarative UI framework for iOS",
                follow_up: vec![deployment_target_step()],
            },
            Choice {
                name: "UIKit",
                description: "Traditional imperative iOS UI framework",
                follow_up: vec![deployment_target_step()],
            },
        ],
    }
}

fn deployment_target_step() -> OptionStep {
    OptionStep {
        title: "iOS Deployment Target",
        choices: vec![
            Choice {
                name: "iOS 17",
                description: "Target recent iOS versions",
                follow_up: vec![],
            },
            Choice {
                name: "iOS 16",
                description: "Wider compatibility for older devices",
                follow_up: vec![],
            },
            Choice {
                name: "iOS 15",
                description: "Maximum compatibility baseline",
                follow_up: vec![],
            },
        ],
    }
}
