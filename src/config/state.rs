#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ConfigField {
    Name,
    Path,
}

pub struct ConfigState {
    pub active_field: ConfigField,
    pub project_name: String,
    pub project_path: String,
    pub error_message: Option<String>,
}

impl ConfigState {
    pub fn new() -> Self {
        ConfigState {
            active_field: ConfigField::Name,
            project_name: String::new(),
            project_path: String::from("./"),
            error_message: None,
        }
    }

    pub fn active_value_mut(&mut self) -> &mut String {
        match self.active_field {
            ConfigField::Name => &mut self.project_name,
            ConfigField::Path => &mut self.project_path,
        }
    }

    pub fn toggle_field(&mut self) {
        self.active_field = match self.active_field {
            ConfigField::Name => ConfigField::Path,
            ConfigField::Path => ConfigField::Name,
        };
    }
}
