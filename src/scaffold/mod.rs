mod nextjs;
mod rust;

use anyhow::Result;
use console::Style;
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::config::{ScaffoldConfig, ScaffoldType};

pub fn run(config: &ScaffoldConfig) -> Result<()> {
    let cwd = std::env::current_dir()?;
    let base_name = Path::new(&config.project_name)
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or(&config.project_name);

    match config.scaffold_type {
        ScaffoldType::Rust => {
            let dir = cwd.join(&config.project_name);
            rust::scaffold(&dir, config.rust.as_ref().unwrap(), base_name)?;
            print_success(&config.project_name, &config.scaffold_type);
            if config.run_install {
                run_install_cargo(&dir)?;
            }
        }
        ScaffoldType::Nextjs => {
            let dir = cwd.join(&config.project_name);
            nextjs::scaffold(&dir, config.nextjs.as_ref().unwrap(), base_name)?;
            print_success(&config.project_name, &config.scaffold_type);
            if config.run_install {
                run_install_npm(&dir)?;
            }
        }
        ScaffoldType::Fullstack => {
            let root = cwd.join(&config.project_name);
            std::fs::create_dir_all(&root)?;

            let api_dir = root.join("api");
            let web_dir = root.join("www");

            let api_name = format!("{}-api", base_name);
            let web_name = format!("{}-web", base_name);

            rust::scaffold(&api_dir, config.rust.as_ref().unwrap(), &api_name)?;
            nextjs::scaffold(&web_dir, config.nextjs.as_ref().unwrap(), &web_name)?;
            write_fullstack_readme(&root, config)?;
            print_success(&config.project_name, &config.scaffold_type);

            if config.run_install {
                run_install_npm(&web_dir)?;
                run_install_cargo(&api_dir)?;
            }
        }
    }

    print_next_steps(config);
    Ok(())
}

fn write_fullstack_readme(root: &Path, config: &ScaffoldConfig) -> Result<()> {
    let content = format!(
        "# {}\n\nFull-stack project scaffolded by create-ruxum-app.\n\n## Structure\n\n- `api/` — Rust Axum backend\n- `www/` — Next.js frontend\n\n## Getting started\n\n```bash\n# Backend\ncd api\ncp .env.example .env\ncargo run\n\n# Frontend\ncd www\ncp .env.example .env.local\nnpm run dev\n```\n",
        config.project_name
    );
    std::fs::write(root.join("README.md"), content)?;
    Ok(())
}

fn run_install_npm(dir: &PathBuf) -> Result<()> {
    let bold = Style::new().bold();
    println!("{}", bold.apply_to("  Running npm install..."));
    let status = Command::new("npm")
        .arg("install")
        .current_dir(dir)
        .status()?;
    if !status.success() {
        eprintln!("  npm install exited with non-zero status");
    }
    Ok(())
}

fn run_install_cargo(dir: &PathBuf) -> Result<()> {
    let bold = Style::new().bold();
    println!("{}", bold.apply_to("  Running cargo build..."));
    let status = Command::new("cargo")
        .arg("build")
        .current_dir(dir)
        .status()?;
    if !status.success() {
        eprintln!("  cargo build exited with non-zero status");
    }
    Ok(())
}

fn print_success(name: &str, scaffold_type: &ScaffoldType) {
    let green = Style::new().green().bold();
    let type_label = match scaffold_type {
        ScaffoldType::Rust => "Rust Axum project",
        ScaffoldType::Nextjs => "Next.js project",
        ScaffoldType::Fullstack => "Full-stack project",
    };
    println!(
        "  {} {} scaffolded successfully!",
        green.apply_to("✔"),
        type_label
    );
    println!("  {} {}/", green.apply_to("→"), name);
    println!();
}

fn print_next_steps(config: &ScaffoldConfig) {
    let bold = Style::new().bold();
    let dim = Style::new().dim();
    println!("{}", bold.apply_to("  Next steps:"));
    println!();

    match config.scaffold_type {
        ScaffoldType::Rust => {
            println!("  {} cd {}", dim.apply_to("$"), config.project_name);
            println!("  {} cp .env.example .env", dim.apply_to("$"));
            if !config.run_install {
                println!("  {} cargo build", dim.apply_to("$"));
            }
            println!("  {} cargo run", dim.apply_to("$"));
        }
        ScaffoldType::Nextjs => {
            println!("  {} cd {}", dim.apply_to("$"), config.project_name);
            println!("  {} cp .env.example .env.local", dim.apply_to("$"));
            if !config.run_install {
                println!("  {} npm install", dim.apply_to("$"));
            }
            println!("  {} npm run dev", dim.apply_to("$"));
        }
        ScaffoldType::Fullstack => {
            println!(
                "  {} cd {}/api && cp .env.example .env",
                dim.apply_to("$"),
                config.project_name
            );
            println!(
                "  {} cd {}/www && cp .env.example .env.local",
                dim.apply_to("$"),
                config.project_name
            );
            if !config.run_install {
                println!(
                    "  {} npm install  {} in www/",
                    dim.apply_to("$"),
                    dim.apply_to("#")
                );
            }
            println!(
                "  {} cargo run   {} in api/",
                dim.apply_to("$"),
                dim.apply_to("#")
            );
            println!(
                "  {} npm run dev  {} in www/",
                dim.apply_to("$"),
                dim.apply_to("#")
            );
        }
    }
    println!();
}

