use std::path::Path;
use std::sync::mpsc::Sender;

use super::command::run_in;
use super::params::ScaffoldParams;
use super::writer::write_file;

pub fn scaffold(params: &ScaffoldParams, base: &Path, tx: &Sender<String>) -> Result<(), String> {
    let pm = params.sel("Package Manager").unwrap_or("pip");
    let project_type = params.sel("Project Type").unwrap_or("");
    let framework = params.sel("Framework");

    match pm {
        "uv" => {
            let _ = tx.send("Running uv init...".to_string());
            run_in(base, "uv", &["init", "."], tx)?;
        }
        "poetry" => {
            let _ = tx.send("Running poetry init...".to_string());
            run_in(base, "poetry", &["init", "--no-interaction"], tx)?;
        }
        "conda" => {
            let _ = tx.send("Writing environment.yml...".to_string());
            write_file(base, "environment.yml", &conda_env(params))?;
        }
        _ => {
            let _ = tx.send("Writing requirements.txt...".to_string());
            write_file(
                base,
                "requirements.txt",
                &requirements(project_type, framework),
            )?;
        }
    }

    let _ = tx.send("Creating main.py...".to_string());
    write_file(base, "main.py", &entry_file(project_type, framework))
}

fn requirements(project_type: &str, framework: Option<&str>) -> String {
    match (project_type, framework) {
        ("Web API", Some("FastAPI")) => "fastapi\nuvicorn[standard]\n".to_string(),
        ("Web API", Some("Flask")) => "flask\n".to_string(),
        ("Web API", Some("Django")) => "django\n".to_string(),
        ("Data Science", _) => "numpy\npandas\nmatplotlib\n".to_string(),
        _ => "# Add your dependencies here\n".to_string(),
    }
}

fn entry_file(project_type: &str, framework: Option<&str>) -> String {
    match (project_type, framework) {
        ("Web API", Some("FastAPI")) => r#"from fastapi import FastAPI

app = FastAPI()

@app.get("/")
def root():
    return {"message": "Hello World!"}
"#
        .to_string(),
        ("Web API", Some("Flask")) => r#"from flask import Flask

app = Flask(__name__)

@app.route("/")
def index():
    return "Hello World!"

if __name__ == "__main__":
    app.run(debug=True)
"#
        .to_string(),
        ("Web API", Some("Django")) => r#"# Run: django-admin startproject myproject .
# Then: python manage.py runserver
print("Run: django-admin startproject myproject .")
"#
        .to_string(),
        ("Data Science", _) => r#"import numpy as np
import pandas as pd

print("Hello, Data Science!")
"#
        .to_string(),
        ("CLI", _) => r#"import argparse

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--name", default="World")
    args = parser.parse_args()
    print(f"Hello, {args.name}!")

if __name__ == "__main__":
    main()
"#
        .to_string(),
        _ => "print(\"Hello World!\")\n".to_string(),
    }
}

fn conda_env(params: &ScaffoldParams) -> String {
    format!(
        "name: {}\nchannels:\n  - defaults\ndependencies:\n  - python>=3.11\n",
        params.project_name
    )
}
