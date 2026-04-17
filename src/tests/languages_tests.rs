use crate::presets::{Category, get_languages};

#[test]
fn all_languages_have_at_least_one_step() {
    for lang in get_languages() {
        assert!(!lang.steps.is_empty(), "{} has no steps", lang.name);
    }
}

#[test]
fn all_steps_have_at_least_one_choice() {
    for lang in get_languages() {
        for step in &lang.steps {
            assert!(
                !step.choices.is_empty(),
                "{} — step '{}' has no choices",
                lang.name,
                step.title
            );
        }
    }
}

#[test]
fn backend_languages_are_correct() {
    let langs: Vec<_> = get_languages()
        .into_iter()
        .filter(|l| l.category == Category::Backend)
        .map(|l| l.name)
        .collect();

    assert!(langs.contains(&"Rust"));
    assert!(langs.contains(&"Go"));
    assert!(langs.contains(&"Java"));
    assert!(langs.contains(&"PHP"));
    assert!(langs.contains(&"Python"));
    assert!(langs.contains(&"TypeScript (Backend)"));
}

#[test]
fn frontend_languages_are_correct() {
    let langs: Vec<_> = get_languages()
        .into_iter()
        .filter(|l| l.category == Category::Frontend)
        .map(|l| l.name)
        .collect();

    assert!(langs.contains(&"TypeScript (Frontend)"));
}

#[test]
fn mobile_languages_are_correct() {
    let langs: Vec<_> = get_languages()
        .into_iter()
        .filter(|l| l.category == Category::Mobile)
        .map(|l| l.name)
        .collect();

    assert!(langs.contains(&"Flutter"));
    assert!(langs.contains(&"Kotlin (Mobile)"));
    assert!(langs.contains(&"Swift (Mobile)"));
}

#[test]
fn desktop_languages_are_correct() {
    let langs: Vec<_> = get_languages()
        .into_iter()
        .filter(|l| l.category == Category::Desktop)
        .map(|l| l.name)
        .collect();

    assert!(langs.contains(&"Swift"));
    assert!(langs.contains(&"Rust (Desktop)"));
    assert!(langs.contains(&"Go (Desktop)"));
    assert!(langs.contains(&"Python (Desktop)"));
}
