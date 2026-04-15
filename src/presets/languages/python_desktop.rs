use super::types::{Category, Choice, Language, OptionStep};

pub fn python_desktop_language() -> Language {
    Language {
        name: "Python (Desktop)",
        category: Category::Desktop,
        steps: vec![OptionStep {
            title: "Framework",
            choices: vec![
                Choice {
                    name: "PyQt6",
                    description: "Qt6 bindings — feature-rich cross-platform GUI",
                    follow_up: vec![],
                },
                Choice {
                    name: "PySide6",
                    description: "Official Qt6 bindings by The Qt Company",
                    follow_up: vec![],
                },
                Choice {
                    name: "Tkinter",
                    description: "Standard library GUI toolkit, no extra deps",
                    follow_up: vec![],
                },
                Choice {
                    name: "wxPython",
                    description: "Native look-and-feel on Windows, macOS, Linux",
                    follow_up: vec![],
                },
                Choice {
                    name: "Kivy",
                    description: "Cross-platform, supports touch and mobile",
                    follow_up: vec![],
                },
            ],
        }],
    }
}
