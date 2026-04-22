#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Category {
    Backend,
    Frontend,
    Mobile,
    Desktop,
}

impl Category {
    pub fn all() -> Vec<Category> {
        vec![
            Category::Backend,
            Category::Frontend,
            Category::Mobile,
            Category::Desktop,
        ]
    }

    pub fn label(&self) -> &'static str {
        match self {
            Category::Backend => "Backend",
            Category::Frontend => "Frontend",
            Category::Mobile => "Mobile",
            Category::Desktop => "Desktop",
        }
    }
}

#[derive(Debug, Clone)]
pub struct OptionStep {
    pub title: &'static str,
    pub choices: Vec<Choice>,
    pub is_multi: bool,
}

impl OptionStep {
    pub fn single(title: &'static str, choices: Vec<Choice>) -> Self {
        Self {
            title,
            choices,
            is_multi: false,
        }
    }

    pub fn multi(title: &'static str, choices: Vec<Choice>) -> Self {
        Self {
            title,
            choices,
            is_multi: true,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Choice {
    pub name: &'static str,
    pub description: &'static str,
    pub follow_up: Vec<OptionStep>,
}

pub struct Language {
    pub name: &'static str,
    pub category: Category,
    pub steps: Vec<OptionStep>,
}
