pub struct ScaffoldParams {
    pub project_path: String,
    pub project_name: String,
    pub language_name: String,
    pub selections: Vec<(String, String)>,
}

impl ScaffoldParams {
    pub fn sel(&self, title: &str) -> Option<&str> {
        self.selections
            .iter()
            .find(|(t, _)| t == title)
            .map(|(_, v)| v.as_str())
    }
}
