use super::shared::{eslint_backend_step, git_hooks_ts_step};
use super::types::{Category, Choice, Language, OptionStep};

fn pm_node_backend() -> OptionStep {
    let eslint = eslint_backend_step();
    let hooks = git_hooks_ts_step();
    OptionStep {
        title: "Package Manager",
        choices: vec![
            Choice {
                name: "npm",
                description: "Default Node.js package manager",
                follow_up: vec![eslint.clone(), hooks.clone()],
            },
            Choice {
                name: "pnpm",
                description: "Fast, disk-efficient package manager",
                follow_up: vec![eslint.clone(), hooks.clone()],
            },
            Choice {
                name: "yarn",
                description: "Reliable JavaScript package manager",
                follow_up: vec![eslint, hooks],
            },
        ],
    }
}

fn pm_bun_backend() -> OptionStep {
    let eslint = eslint_backend_step();
    let hooks = git_hooks_ts_step();
    OptionStep {
        title: "Package Manager",
        choices: vec![
            Choice {
                name: "bun",
                description: "Bun's built-in package manager",
                follow_up: vec![eslint.clone(), hooks.clone()],
            },
            Choice {
                name: "npm",
                description: "Node.js package manager",
                follow_up: vec![eslint.clone(), hooks.clone()],
            },
            Choice {
                name: "pnpm",
                description: "Fast, disk-efficient package manager",
                follow_up: vec![eslint, hooks],
            },
        ],
    }
}

fn pm_deno_backend() -> OptionStep {
    let eslint = eslint_backend_step();
    let hooks = git_hooks_ts_step();
    OptionStep {
        title: "Package Manager",
        choices: vec![
            Choice {
                name: "deno",
                description: "Deno's built-in package manager",
                follow_up: vec![eslint.clone(), hooks.clone()],
            },
            Choice {
                name: "npm",
                description: "Node.js package manager",
                follow_up: vec![eslint.clone(), hooks.clone()],
            },
            Choice {
                name: "pnpm",
                description: "Fast, disk-efficient package manager",
                follow_up: vec![eslint, hooks],
            },
        ],
    }
}

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
                            Choice {
                                name: "Express",
                                description: "Minimal and flexible web framework",
                                follow_up: vec![pm_node_backend()],
                            },
                            Choice {
                                name: "Fastify",
                                description: "Fast and low-overhead web framework",
                                follow_up: vec![pm_node_backend()],
                            },
                            Choice {
                                name: "NestJS",
                                description: "Progressive Node.js framework",
                                follow_up: vec![pm_node_backend()],
                            },
                            Choice {
                                name: "Hono",
                                description: "Ultrafast web framework",
                                follow_up: vec![pm_node_backend()],
                            },
                        ],
                    }],
                },
                Choice {
                    name: "Bun",
                    description: "Fast all-in-one JavaScript toolkit",
                    follow_up: vec![OptionStep {
                        title: "Framework",
                        choices: vec![
                            Choice {
                                name: "Hono",
                                description: "Ultrafast web framework",
                                follow_up: vec![pm_bun_backend()],
                            },
                            Choice {
                                name: "Elysia",
                                description: "Ergonomic framework for Bun",
                                follow_up: vec![pm_bun_backend()],
                            },
                            Choice {
                                name: "Express",
                                description: "Minimal and flexible web framework",
                                follow_up: vec![pm_bun_backend()],
                            },
                        ],
                    }],
                },
                Choice {
                    name: "Deno",
                    description: "Secure runtime for JavaScript and TypeScript",
                    follow_up: vec![OptionStep {
                        title: "Framework",
                        choices: vec![
                            Choice {
                                name: "Fresh",
                                description: "Next-gen web framework for Deno",
                                follow_up: vec![pm_deno_backend()],
                            },
                            Choice {
                                name: "Oak",
                                description: "Middleware framework inspired by Koa",
                                follow_up: vec![pm_deno_backend()],
                            },
                            Choice {
                                name: "Hono",
                                description: "Ultrafast web framework",
                                follow_up: vec![pm_deno_backend()],
                            },
                        ],
                    }],
                },
            ],
        }],
    }
}
