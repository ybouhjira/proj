use anyhow::{Context, Result};
use console::style;
use tokio::process::Command;

use crate::config::Config;
use crate::discovery;
use crate::fuzzy;

pub async fn execute(name: &str, github: bool, dir: bool) -> Result<()> {
    let config = Config::load()?;
    let projects_dir = config.projects_dir_expanded();

    let local_projects = discovery::discover_local(&projects_dir).await?;
    let remote_projects = discovery::discover_remote(&config).await?;
    let projects = discovery::merge_projects(local_projects, remote_projects).await;

    let project_names: Vec<String> = projects.iter().map(|p| p.name.clone()).collect();
    let matches = fuzzy::fuzzy_match(name, &project_names);

    if matches.is_empty() {
        anyhow::bail!("No projects match '{}'", name);
    }

    let project_name = &matches[0].0;
    let project = projects
        .iter()
        .find(|p| &p.name == project_name)
        .context("Project not found")?;

    if github {
        if let Some(ref repo) = project.github_repo {
            println!(
                "{} Opening {} in browser...",
                style("→").cyan(),
                style(&repo.url).dim()
            );

            let open_cmd = if cfg!(target_os = "macos") {
                "open"
            } else if cfg!(target_os = "linux") {
                "xdg-open"
            } else {
                "start"
            };

            Command::new(open_cmd)
                .arg(&repo.url)
                .spawn()
                .context("Failed to open browser")?;

            return Ok(());
        } else {
            anyhow::bail!("Project '{}' has no GitHub repository", project_name);
        }
    }

    if dir {
        if let Some(ref path) = project.local_path {
            let open_cmd = if cfg!(target_os = "macos") {
                "open"
            } else if cfg!(target_os = "linux") {
                "xdg-open"
            } else {
                "explorer"
            };

            Command::new(open_cmd)
                .arg(path)
                .spawn()
                .context("Failed to open file manager")?;

            println!(
                "{} Opened {} in file manager",
                style("✓").green(),
                style(path.display()).cyan()
            );

            return Ok(());
        } else {
            anyhow::bail!("Project '{}' is not cloned locally", project_name);
        }
    }

    if let Some(ref path) = project.local_path {
        println!(
            "{} Opening {} in {}...",
            style("→").cyan(),
            style(project_name).bold(),
            style(&config.editor).dim()
        );

        Command::new(&config.editor)
            .arg(path)
            .spawn()
            .context("Failed to open editor")?;

        Ok(())
    } else {
        anyhow::bail!(
            "Project '{}' is not cloned locally. Use 'proj clone {}' first.",
            project_name,
            project_name
        );
    }
}
