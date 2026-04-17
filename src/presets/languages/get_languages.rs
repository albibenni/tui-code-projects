use super::flutter::flutter_language;
use super::go::go_language;
use super::go_desktop::go_desktop_language;
use super::java::java_language;
use super::php::php_language;
use super::python::python_language;
use super::python_desktop::python_desktop_language;
use super::rust::rust_language;
use super::rust_desktop::rust_desktop_language;
use super::swift::swift_language;
use super::types::Language;
use super::typescript_backend::typescript_backend_language;
use super::typescript_frontend::typescript_frontend_language;

pub fn get_languages() -> Vec<Language> {
    vec![
        rust_language(),
        go_language(),
        java_language(),
        php_language(),
        python_language(),
        typescript_backend_language(),
        typescript_frontend_language(),
        flutter_language(),
        swift_language(),
        rust_desktop_language(),
        go_desktop_language(),
        python_desktop_language(),
    ]
}
