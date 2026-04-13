use super::go::go_language;
use super::python::python_language;
use super::rust::rust_language;
use super::typescript_backend::typescript_backend_language;
use super::typescript_frontend::typescript_frontend_language;
use super::types::Language;

pub fn get_languages() -> Vec<Language> {
    vec![
        rust_language(),
        go_language(),
        python_language(),
        typescript_backend_language(),
        typescript_frontend_language(),
    ]
}
