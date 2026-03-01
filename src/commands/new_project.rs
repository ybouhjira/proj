use anyhow::{Context, Result};
use console::style;
use tokio::process::Command;

use crate::config::Config;
use crate::github;

pub async fn execute(name: &str, public: bool, lang: Option<String>) -> Result<()> {
    let config = Config::load()?;
    let projects_dir = config.projects_dir_expanded();
    let project_path = projects_dir.join(name);

    if project_path.exists() {
        anyhow::bail!("Directory {} already exists", project_path.display());
    }

    println!(
        "{} Creating project '{}'...",
        style("→").cyan(),
        style(name).bold()
    );

    std::fs::create_dir_all(&project_path).context("Failed to create project directory")?;

    println!("  {} Created directory", style("✓").green());

    let git_init = Command::new("git")
        .args(["init"])
        .current_dir(&project_path)
        .output()
        .await
        .context("Failed to initialize git")?;

    if !git_init.status.success() {
        anyhow::bail!("git init failed");
    }

    println!("  {} Initialized git repository", style("✓").green());

    if let Some(language) = lang {
        create_gitignore(&project_path, &language)?;
        println!(
            "  {} Created .gitignore for {}",
            style("✓").green(),
            language
        );
    }

    println!("  {} Creating GitHub repository...", style("→").cyan());

    github::create_repo(name, !public)
        .await
        .context("Failed to create GitHub repository")?;

    println!(
        "  {} Created GitHub repository ({})",
        style("✓").green(),
        if public { "public" } else { "private" }
    );

    let remote_url = format!("git@github.com:{}/{}.git", config.github_user, name);
    let add_remote = Command::new("git")
        .args(["remote", "add", "origin", &remote_url])
        .current_dir(&project_path)
        .output()
        .await
        .context("Failed to add git remote")?;

    if !add_remote.status.success() {
        eprintln!(
            "  {} Warning: Could not add git remote",
            style("⚠").yellow()
        );
    } else {
        println!("  {} Added git remote", style("✓").green());
    }

    println!();
    println!(
        "{} Project '{}' created at {}",
        style("✓").green().bold(),
        style(name).bold(),
        style(project_path.display()).cyan()
    );
    println!();
    println!("  Next steps:");
    println!("    cd {}", project_path.display());
    println!("    # ... create files ...");
    println!("    git add .");
    println!("    git commit -m \"Initial commit\"");
    println!("    git push -u origin main");
    println!();

    Ok(())
}

fn create_gitignore(project_path: &std::path::Path, language: &str) -> Result<()> {
    let gitignore_content = match language {
        "rust" => include_str!("../../templates/.gitignore.rust"),
        "js" | "javascript" | "typescript" | "ts" => include_str!("../../templates/.gitignore.js"),
        "py" | "python" => include_str!("../../templates/.gitignore.py"),
        "go" => include_str!("../../templates/.gitignore.go"),
        _ => return Ok(()),
    };

    let gitignore_path = project_path.join(".gitignore");
    std::fs::write(gitignore_path, gitignore_content)?;

    Ok(())
}
