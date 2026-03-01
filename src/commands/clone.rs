use anyhow::{Context, Result};
use console::style;
use indicatif::{ProgressBar, ProgressStyle};
use tokio::process::Command;

use crate::config::Config;
use crate::discovery;
use crate::fuzzy;

pub async fn execute(name: &str) -> Result<()> {
    let config = Config::load()?;
    let projects_dir = config.projects_dir_expanded();

    let spinner = ProgressBar::new_spinner();
    spinner.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.cyan} {msg}")
            .unwrap()
    );
    spinner.set_message("Fetching remote repositories...");
    spinner.enable_steady_tick(std::time::Duration::from_millis(100));

    let remote_projects = discovery::discover_remote(&config).await?;

    let repo_names: Vec<String> = remote_projects
        .iter()
        .map(|p| p.name.clone())
        .collect();

    let matches = fuzzy::fuzzy_match(name, &repo_names);

    if matches.is_empty() {
        spinner.finish_and_clear();
        eprintln!("{} No remote repositories match '{}'", style("✗").red(), name);
        std::process::exit(1);
    }

    let repo_name = &matches[0].0;
    let project = remote_projects.iter()
        .find(|p| &p.name == repo_name)
        .context("Repository not found")?;

    let github_repo = project.github_repo.as_ref()
        .context("No GitHub repository info")?;

    spinner.finish_and_clear();

    println!("{} Cloning {} to {}...",
        style("→").cyan(),
        style(&github_repo.full_name).bold(),
        style(projects_dir.display()).dim()
    );

    std::fs::create_dir_all(&projects_dir)?;

    let clone_path = projects_dir.join(repo_name);

    let output = Command::new("git")
        .args(["clone", &github_repo.url, clone_path.to_str().unwrap()])
        .output()
        .await
        .context("Failed to clone repository")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("Clone failed: {}", stderr);
    }

    println!("{} Cloned to {}",
        style("✓").green().bold(),
        style(clone_path.display()).cyan()
    );

    Ok(())
}
