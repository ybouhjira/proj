use anyhow::{Context, Result};
use console::style;

use crate::config::Config;
use crate::discovery;
use crate::fuzzy;

pub async fn execute(query: &str) -> Result<()> {
    let config = Config::load()?;
    let projects_dir = config.projects_dir_expanded();

    let projects = discovery::discover_local(&projects_dir).await?;

    let local_names: Vec<String> = projects
        .iter()
        .filter(|p| p.local_path.is_some())
        .map(|p| p.name.clone())
        .collect();

    if local_names.is_empty() {
        anyhow::bail!("No local projects found");
    }

    let matches = fuzzy::fuzzy_match(query, &local_names);

    if matches.is_empty() {
        eprintln!("{} No projects match '{}'", style("✗").red(), query);
        std::process::exit(1);
    }

    if matches.len() == 1 || (matches.len() > 1 && matches[0].1 > matches[1].1 * 2) {
        let project_name = &matches[0].0;
        let project = projects.iter()
            .find(|p| &p.name == project_name)
            .context("Project not found")?;

        if let Some(path) = &project.local_path {
            println!("{}", path.display());
            return Ok(());
        }
    }

    eprintln!("{} Ambiguous query '{}'. Did you mean:", style("?").yellow(), query);
    for (name, score) in matches.iter().take(3) {
        eprintln!("  {} {}", style("•").dim(), style(name).cyan().dim().to_string() + &format!(" (score: {})", score));
    }
    std::process::exit(1);
}
