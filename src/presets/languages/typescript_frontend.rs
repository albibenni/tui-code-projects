use super::shared::{eslint_frontend_step, pm_js};
use super::types::{Category, Choice, Language, OptionStep};

fn angular_pm_step() -> OptionStep {
    let eslint = eslint_frontend_step();
    OptionStep::single(
        "Package Manager",
        vec![
            Choice {
                name: "npm",
                description: "Default Node.js package manager",
                follow_up: vec![eslint.clone()],
            },
            Choice {
                name: "pnpm",
                description: "Fast, disk-efficient package manager",
                follow_up: vec![eslint],
            },
        ],
    )
}

fn pm_js_with_libs(libs: OptionStep) -> OptionStep {
    let eslint = eslint_frontend_step_with_libs(libs);
    OptionStep::single(
        "Package Manager",
        vec![
            Choice {
                name: "npm",
                description: "Default Node.js package manager",
                follow_up: vec![eslint.clone()],
            },
            Choice {
                name: "pnpm",
                description: "Fast, disk-efficient package manager",
                follow_up: vec![eslint.clone()],
            },
            Choice {
                name: "bun",
                description: "Fast JavaScript all-in-one toolkit",
                follow_up: vec![eslint],
            },
        ],
    )
}

fn eslint_frontend_step_with_libs(libs: OptionStep) -> OptionStep {
    OptionStep::single(
        "ESLint",
        vec![
            Choice {
                name: "None",
                description: "Skip ESLint",
                follow_up: vec![libs.clone()],
            },
            Choice {
                name: "Recommended",
                description: "typescript-eslint recommended, flat config (eslint.config.js)",
                follow_up: vec![libs.clone()],
            },
            Choice {
                name: "Recommended + Prettier",
                description: "Recommended + eslint-plugin-prettier, flat config",
                follow_up: vec![libs.clone()],
            },
            Choice {
                name: "Custom Strict",
                description: "Full preset: eslint.config.js + tsconfig.json + .prettierrc",
                follow_up: vec![libs],
            },
        ],
    )
}

pub fn react_libraries_step() -> OptionStep {
    let hooks = super::shared::git_hooks_ts_step();
    OptionStep::multi(
        "Libraries",
        vec![
            Choice {
                name: "TanStack Query",
                description: "Powerful asynchronous state management",
                follow_up: vec![hooks.clone()],
            },
            Choice {
                name: "Tailwind CSS",
                description: "Utility-first CSS framework",
                follow_up: vec![hooks.clone()],
            },
            Choice {
                name: "Lucide React",
                description: "Beautifully simple pixel-perfect icons",
                follow_up: vec![hooks],
            },
        ],
    )
}

pub fn typescript_frontend_language() -> Language {
    let lib_step = react_libraries_step();
    Language {
        name: "TypeScript (Frontend)",
        category: Category::Frontend,
        steps: vec![OptionStep::single(
            "Framework",
            vec![
                Choice {
                    name: "React",
                    description: "UI library for building interfaces",
                    follow_up: vec![OptionStep::single(
                        "Variant",
                        vec![
                            Choice {
                                name: "Vite",
                                description: "Fast SPA setup with Vite",
                                follow_up: vec![pm_js_with_libs(lib_step.clone())],
                            },
                            Choice {
                                name: "Next.js",
                                description: "React SSR/SSG framework",
                                follow_up: vec![pm_js_with_libs(lib_step.clone())],
                            },
                            Choice {
                                name: "Remix",
                                description: "Full-stack React framework",
                                follow_up: vec![pm_js_with_libs(lib_step.clone())],
                            },
                            Choice {
                                name: "TanStack Start",
                                description: "Full-stack React framework by TanStack",
                                follow_up: vec![pm_js_with_libs(lib_step.clone())],
                            },
                            Choice {
                                name: "Expo",
                                description: "React Native cross-platform",
                                follow_up: vec![pm_js_with_libs(lib_step)],
                            },
                        ],
                    )],
                },
                Choice {
                    name: "Vue",
                    description: "Progressive JavaScript framework",
                    follow_up: vec![OptionStep::single(
                        "Variant",
                        vec![
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
                    )],
                },
                Choice {
                    name: "Svelte",
                    description: "Cybernetically enhanced web apps",
                    follow_up: vec![OptionStep::single(
                        "Variant",
                        vec![
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
                    )],
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
        )],
    }
}
