#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Category {
    Backend,
    Frontend,
}

impl Category {
    pub fn all() -> Vec<Category> {
        vec![Category::Backend, Category::Frontend]
    }

    pub fn label(&self) -> &'static str {
        match self {
            Category::Backend => "Backend",
            Category::Frontend => "Frontend",
        }
    }
}

#[derive(Debug, Clone)]
pub struct OptionStep {
    pub title: &'static str,
    pub choices: Vec<Choice>,
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
