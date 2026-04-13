use super::shared::{pm_bun, pm_deno, pm_node};
use super::types::{Category, Choice, Language, OptionStep};

pub fn typescript_backend_language() -> Language {
    Language {
        name: "TypeScript (Backend)",
        category: Category::Backend,
        steps: vec![OptionStep {
            title: "Runtime",
            choices: vec![
                Choice {
                    name: "Node",
                    description: "Node.js runtime",
                    follow_up: vec![OptionStep {
                        title: "Framework",
                        choices: vec![
                            Choice { name: "Express", description: "Minimal and flexible web framework",  follow_up: vec![pm_node()] },
                            Choice { name: "Fastify", description: "Fast and low-overhead web framework", follow_up: vec![pm_node()] },
                            Choice { name: "NestJS",  description: "Progressive Node.js framework",       follow_up: vec![pm_node()] },
                            Choice { name: "Hono",    description: "Ultrafast web framework",             follow_up: vec![pm_node()] },
                        ],
                    }],
                },
                Choice {
                    name: "Bun",
                    description: "Fast all-in-one JavaScript toolkit",
                    follow_up: vec![OptionStep {
                        title: "Framework",
                        choices: vec![
                            Choice { name: "Hono",    description: "Ultrafast web framework",             follow_up: vec![pm_bun()] },
                            Choice { name: "Elysia",  description: "Ergonomic framework for Bun",         follow_up: vec![pm_bun()] },
                            Choice { name: "Express", description: "Minimal and flexible web framework",  follow_up: vec![pm_bun()] },
                        ],
                    }],
                },
                Choice {
                    name: "Deno",
                    description: "Secure runtime for JavaScript and TypeScript",
                    follow_up: vec![OptionStep {
                        title: "Framework",
                        choices: vec![
                            Choice { name: "Fresh", description: "Next-gen web framework for Deno",       follow_up: vec![pm_deno()] },
                            Choice { name: "Oak",   description: "Middleware framework inspired by Koa",  follow_up: vec![pm_deno()] },
                            Choice { name: "Hono",  description: "Ultrafast web framework",               follow_up: vec![pm_deno()] },
                        ],
                    }],
                },
            ],
        }],
    }
}
