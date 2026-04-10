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

pub struct Preset {
    pub name: &'static str,
    pub description: &'static str,
    pub command: &'static str,
}

pub struct Language {
    pub name: &'static str,
    pub category: Category,
    pub presets: Vec<Preset>,
}

pub fn get_languages() -> Vec<Language> {
    vec![
        Language {
            name: "Rust",
            category: Category::Backend,
            presets: vec![
                Preset {
                    name: "Binary",
                    description: "Executable binary crate",
                    command: "cargo init",
                },
                Preset {
                    name: "Library",
                    description: "Reusable library crate",
                    command: "cargo init --lib",
                },
            ],
        },
        Language {
            name: "Go",
            category: Category::Backend,
            presets: vec![Preset {
                name: "Module",
                description: "Standard Go module",
                command: "go mod init",
            }],
        },
        Language {
            name: "Python",
            category: Category::Backend,
            presets: vec![
                Preset {
                    name: "Script",
                    description: "Simple standalone script",
                    command: "mkdir -p src",
                },
                Preset {
                    name: "FastAPI",
                    description: "Async REST API with FastAPI",
                    command: "pip install fastapi uvicorn",
                },
                Preset {
                    name: "CLI",
                    description: "Command-line tool with argparse",
                    command: "mkdir -p src",
                },
            ],
        },
        Language {
            name: "TypeScript (Backend)",
            category: Category::Backend,
            presets: vec![
                Preset {
                    name: "Node + Express",
                    description: "Minimal REST API",
                    command: "npm init -y && npm install express typescript @types/express ts-node",
                },
                Preset {
                    name: "Fastify",
                    description: "Fast and low-overhead web framework",
                    command: "npm init -y && npm install fastify typescript ts-node",
                },
                Preset {
                    name: "Node CLI",
                    description: "Standalone CLI tool",
                    command: "npm init -y && npm install typescript ts-node @types/node",
                },
                Preset {
                    name: "Bun + Hono",
                    description: "Bun runtime with lightweight Hono framework",
                    command: "bun create hono",
                },
            ],
        },
        Language {
            name: "TypeScript (Frontend)",
            category: Category::Frontend,
            presets: vec![
                Preset {
                    name: "Vite + React",
                    description: "SPA with React and fast dev server",
                    command: "npm create vite@latest -- --template react-ts",
                },
                Preset {
                    name: "Vite + React + SWC",
                    description: "React with faster SWC compiler",
                    command: "npm create vite@latest -- --template react-swc-ts",
                },
                Preset {
                    name: "Vite + Vanilla",
                    description: "No framework, plain TypeScript",
                    command: "npm create vite@latest -- --template vanilla-ts",
                },
                Preset {
                    name: "Next.js",
                    description: "React SSR/SSG framework",
                    command: "npx create-next-app@latest",
                },
            ],
        },
    ]
}
