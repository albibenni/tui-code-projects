use std::fs;
use std::path::Path;
use std::sync::mpsc::Sender;

use super::params::ScaffoldParams;
use super::writer::write_file;

pub fn scaffold(params: &ScaffoldParams, base: &Path, tx: &Sender<String>) -> Result<(), String> {
    let project_type = params.sel("Project Type").unwrap_or("CLI");
    let framework = params.sel("Framework");
    let build_tool = params.sel("Build Tool").unwrap_or("Maven");

    let _ = tx.send("Writing Java project files...".to_string());

    fs::create_dir_all(base.join("src")).map_err(|e| format!("Failed to create src/: {e}"))?;

    write_file(
        base,
        "README.md",
        &readme(project_type, framework, build_tool),
    )?;
    write_file(base, "src/Main.java", &main_java(project_type, framework))?;

    match build_tool {
        "Gradle" => {
            write_file(base, "settings.gradle", "rootProject.name = 'app'\n")?;
            write_file(base, "build.gradle", &gradle_file(project_type, framework))?;
            write_file(base, "Makefile", makefile(build_tool))?;
        }
        _ => {
            write_file(base, "pom.xml", &pom_xml(params, project_type, framework))?;
            write_file(base, "Makefile", makefile(build_tool))?;
        }
    }

    Ok(())
}

fn makefile(build_tool: &str) -> &'static str {
    match build_tool {
        "Gradle" => {
            r#"GRADLE ?= gradle

.PHONY: build run test lint clean

build:
	@$(GRADLE) build

run:
	@$(GRADLE) run || true

test:
	@$(GRADLE) test

lint:
	@echo "No linter configured for this Java preset."

clean:
	@$(GRADLE) clean
"#
        }
        _ => {
            r#"MAVEN ?= mvn

.PHONY: build run test lint clean

build:
	@$(MAVEN) -q package

run:
	@$(MAVEN) -q exec:java || true

test:
	@$(MAVEN) -q test

lint:
	@echo "No linter configured for this Java preset."

clean:
	@$(MAVEN) -q clean
"#
        }
    }
}

fn readme(project_type: &str, framework: Option<&str>, build_tool: &str) -> String {
    match (project_type, framework) {
        ("Web API", Some("Spring Boot")) => format!(
            "# Java API\n\n- Framework: Spring Boot\n- Build Tool: {build_tool}\n\nRun with your preferred build command.\n"
        ),
        ("Web API", Some("Micronaut")) => format!(
            "# Java API\n\n- Framework: Micronaut\n- Build Tool: {build_tool}\n\nRun with your preferred build command.\n"
        ),
        ("Web API", Some("Javalin")) => format!(
            "# Java API\n\n- Framework: Javalin\n- Build Tool: {build_tool}\n\nRun with your preferred build command.\n"
        ),
        _ => format!(
            "# Java CLI\n\n- Build Tool: {build_tool}\n\nRun with your preferred build command.\n"
        ),
    }
}

fn main_java(project_type: &str, framework: Option<&str>) -> String {
    match (project_type, framework) {
        ("Web API", Some("Spring Boot")) => r#"import org.springframework.boot.SpringApplication;
import org.springframework.boot.autoconfigure.SpringBootApplication;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.RestController;

@SpringBootApplication
public class Main {
    public static void main(String[] args) {
        SpringApplication.run(Main.class, args);
    }
}

@RestController
class HelloController {
    @GetMapping("/")
    String index() {
        return "Hello World!";
    }
}
"#
        .to_string(),
        ("Web API", Some("Micronaut")) => r#"import io.micronaut.http.annotation.Controller;
import io.micronaut.http.annotation.Get;
import io.micronaut.runtime.Micronaut;

public class Main {
    public static void main(String[] args) {
        Micronaut.run(Main.class, args);
    }
}

@Controller("/")
class HelloController {
    @Get
    String index() {
        return "Hello World!";
    }
}
"#
        .to_string(),
        ("Web API", Some("Javalin")) => r#"import io.javalin.Javalin;

public class Main {
    public static void main(String[] args) {
        var app = Javalin.create().start(3000);
        app.get("/", ctx -> ctx.result("Hello World!"));
    }
}
"#
        .to_string(),
        _ => r#"public class Main {
    public static void main(String[] args) {
        System.out.println("Hello World!");
    }
}
"#
        .to_string(),
    }
}

fn gradle_file(project_type: &str, framework: Option<&str>) -> String {
    let deps = match (project_type, framework) {
        ("Web API", Some("Spring Boot")) => {
            "dependencies {\n    implementation 'org.springframework.boot:spring-boot-starter-web:3.5.0'\n}\n"
        }
        ("Web API", Some("Micronaut")) => {
            "dependencies {\n    implementation 'io.micronaut:micronaut-http-server-netty:4.8.0'\n    annotationProcessor 'io.micronaut:micronaut-inject-java:4.8.0'\n}\n"
        }
        ("Web API", Some("Javalin")) => {
            "dependencies {\n    implementation 'io.javalin:javalin:6.3.0'\n}\n"
        }
        _ => "dependencies {}\n",
    };

    format!("plugins {{\n    id 'java'\n}}\n\nrepositories {{\n    mavenCentral()\n}}\n\n{deps}")
}

fn pom_xml(params: &ScaffoldParams, project_type: &str, framework: Option<&str>) -> String {
    let deps = match (project_type, framework) {
        ("Web API", Some("Spring Boot")) => {
            "    <dependency>\n      <groupId>org.springframework.boot</groupId>\n      <artifactId>spring-boot-starter-web</artifactId>\n      <version>3.5.0</version>\n    </dependency>\n"
        }
        ("Web API", Some("Micronaut")) => {
            "    <dependency>\n      <groupId>io.micronaut</groupId>\n      <artifactId>micronaut-http-server-netty</artifactId>\n      <version>4.8.0</version>\n    </dependency>\n"
        }
        ("Web API", Some("Javalin")) => {
            "    <dependency>\n      <groupId>io.javalin</groupId>\n      <artifactId>javalin</artifactId>\n      <version>6.3.0</version>\n    </dependency>\n"
        }
        _ => "",
    };

    format!(
        r#"<project xmlns="http://maven.apache.org/POM/4.0.0"
         xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
         xsi:schemaLocation="http://maven.apache.org/POM/4.0.0 http://maven.apache.org/xsd/maven-4.0.0.xsd">
  <modelVersion>4.0.0</modelVersion>
  <groupId>com.example</groupId>
  <artifactId>{}</artifactId>
  <version>0.1.0</version>
{deps}</project>
"#,
        params.project_name
    )
}
