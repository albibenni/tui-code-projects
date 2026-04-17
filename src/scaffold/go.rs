use std::fs;
use std::path::Path;
use std::sync::mpsc::Sender;

use super::command::run_in;
use super::params::ScaffoldParams;
use super::writer::write_file;

pub fn scaffold(params: &ScaffoldParams, base: &Path, tx: &Sender<String>) -> Result<(), String> {
    let _ = tx.send(format!("Running go mod init {}...", params.project_name));
    run_in(base, "go", &["mod", "init", &params.project_name], tx)?;

    let framework = params.sel("Framework");
    let project_type = params.sel("Project Type").unwrap_or("");

    let _ = tx.send("Creating main.go...".to_string());
    write_main_go(base, project_type, framework)
}

fn write_main_go(base: &Path, project_type: &str, framework: Option<&str>) -> Result<(), String> {
    fs::create_dir_all(base).map_err(|e| format!("Failed to create directory: {e}"))?;

    let content = match (project_type, framework) {
        ("Web API", Some("Gin")) => {
            r#"package main

import "github.com/gin-gonic/gin"

func main() {
	r := gin.Default()
	r.GET("/", func(c *gin.Context) {
		c.JSON(200, gin.H{"message": "Hello World!"})
	})
	r.Run(":3000")
}
"#
        }
        ("Web API", Some("Echo")) => {
            r#"package main

import (
	"net/http"

	"github.com/labstack/echo/v4"
)

func main() {
	e := echo.New()
	e.GET("/", func(c echo.Context) error {
		return c.String(http.StatusOK, "Hello World!")
	})
	e.Logger.Fatal(e.Start(":3000"))
}
"#
        }
        ("Web API", Some("Fiber")) => {
            r#"package main

import "github.com/gofiber/fiber/v2"

func main() {
	app := fiber.New()
	app.Get("/", func(c *fiber.Ctx) error {
		return c.SendString("Hello World!")
	})
	app.Listen(":3000")
}
"#
        }
        ("Web API", Some("Chi")) => {
            r#"package main

import (
	"fmt"
	"net/http"

	"github.com/go-chi/chi/v5"
)

func main() {
	r := chi.NewRouter()
	r.Get("/", func(w http.ResponseWriter, r *http.Request) {
		fmt.Fprintln(w, "Hello World!")
	})
	http.ListenAndServe(":3000", r)
}
"#
        }
        ("Web API", _) => {
            r#"package main

import (
	"fmt"
	"net/http"
)

func main() {
	http.HandleFunc("/", func(w http.ResponseWriter, r *http.Request) {
		fmt.Fprintln(w, "Hello World!")
	})
	http.ListenAndServe(":3000", nil)
}
"#
        }
        ("Library", _) => {
            r#"package main

// Package main is the entry point for the library.
func main() {}
"#
        }
        _ => {
            r#"package main

import "fmt"

func main() {
	fmt.Println("Hello World!")
}
"#
        }
    };

    write_file(base, "main.go", content)
}
