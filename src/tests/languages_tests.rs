use crate::presets::{Category, get_languages};

#[test]
fn all_languages_have_at_least_one_preset() {
    for lang in get_languages() {
        assert!(!lang.presets.is_empty(), "{} has no presets", lang.name);
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
