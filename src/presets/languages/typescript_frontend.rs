use super::shared::{git_hooks_ts_step, pm_js};
use super::types::{Category, Choice, Language, OptionStep};

fn angular_pm_step() -> OptionStep {
    let hooks = git_hooks_ts_step();
    OptionStep {
        title: "Package Manager",
        choices: vec![
            Choice {
                name: "npm",
                description: "Default Node.js package manager",
                follow_up: vec![hooks.clone()],
            },
            Choice {
                name: "pnpm",
                description: "Fast, disk-efficient package manager",
                follow_up: vec![hooks],
            },
        ],
    }
}

pub fn typescript_frontend_language() -> Language {
    Language {
        name: "TypeScript (Frontend)",
        category: Category::Frontend,
        steps: vec![OptionStep {
            title: "Framework",
            choices: vec![
                Choice {
                    name: "React",
                    description: "UI library for building interfaces",
                    follow_up: vec![OptionStep {
                        title: "Variant",
                        choices: vec![
                            Choice {
                                name: "Vite",
                                description: "Fast SPA setup with Vite",
                                follow_up: vec![pm_js()],
                            },
                            Choice {
                                name: "Next.js",
                                description: "React SSR/SSG framework",
                                follow_up: vec![pm_js()],
                            },
                            Choice {
                                name: "Remix",
                                description: "Full-stack React framework",
                                follow_up: vec![pm_js()],
                            },
                            Choice {
                                name: "TanStack Start",
                                description: "Full-stack React framework by TanStack",
                                follow_up: vec![pm_js()],
                            },
                            Choice {
                                name: "Expo",
                                description: "React Native cross-platform",
                                follow_up: vec![pm_js()],
                            },
                        ],
                    }],
                },
                Choice {
                    name: "Vue",
                    description: "Progressive JavaScript framework",
                    follow_up: vec![OptionStep {
                        title: "Variant",
                        choices: vec![
                            Choice {
                                name: "Vite",
                                description: "Fast SPA setup with Vite",
                                follow_up: vec![pm_js()],
                            },
                            Choice {
                                name: "Nuxt",
                                description: "Full-stack Vue framework",
                                follow_up: vec![pm_js()],
                            },
                        ],
                    }],
                },
                Choice {
                    name: "Svelte",
                    description: "Cybernetically enhanced web apps",
                    follow_up: vec![OptionStep {
                        title: "Variant",
                        choices: vec![
                            Choice {
                                name: "SvelteKit",
                                description: "Full-stack Svelte framework",
                                follow_up: vec![pm_js()],
                            },
                            Choice {
                                name: "Vite",
                                description: "Lightweight SPA setup",
                                follow_up: vec![pm_js()],
                            },
                        ],
                    }],
                },
                Choice {
                    name: "Angular",
                    description: "Platform for building web applications",
                    follow_up: vec![angular_pm_step()],
                },
                Choice {
                    name: "Astro",
                    description: "Content-focused web framework",
                    follow_up: vec![pm_js()],
                },
                Choice {
                    name: "Qwik",
                    description: "Resumable framework for instant apps",
                    follow_up: vec![pm_js()],
                },
                Choice {
                    name: "Solid",
                    description: "Reactive UI library without a virtual DOM",
                    follow_up: vec![pm_js()],
                },
            ],
        }],
    }
}
