use anyhow::{Context, Result};
use console::style;

use crate::config::Config;
use crate::discovery;
use crate::fuzzy;
use crate::ui;

pub async fn execute(name: &str) -> Result<()> {
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
    let project = projects.iter()
        .find(|p| &p.name == project_name)
        .context("Project not found")?;

    println!();
    println!("  {} {}", style("📦").bold(), style(&project.name).bold().cyan());
    println!();

    if let Some(ref path) = project.local_path {
        println!("  {} {}", style("Path:").bold(), path.display());
    }

    if let Some(ref git_status) = project.git_status {
        println!("  {} {}", style("Branch:").bold(), git_status.branch);
        println!("  {} {}", style("Dirty files:").bold(),
            if git_status.dirty_files > 0 {
                style(git_status.dirty_files).yellow().to_string()
            } else {
                style("0").green().to_string()
            }
        );
        println!("  {} {}", style("Last commit:").bold(), git_status.last_commit_msg);
    }

    if let Some(ref repo) = project.github_repo {
        println!();
        println!("  {} GitHub", style("🔗").bold());
        println!("  {} {}", style("URL:").bold(), repo.url);
        if let Some(ref desc) = repo.description {
            println!("  {} {}", style("Description:").bold(), desc);
        }
        if let Some(ref lang) = repo.language {
            println!("  {} {}", style("Language:").bold(), lang);
        }
        println!("  {} {}", style("Stars:").bold(), repo.stars);
        println!("  {} {}", style("Private:").bold(),
            if repo.is_private {
                style("yes").yellow()
            } else {
                style("no").green()
            }
        );
        println!("  {} {}", style("Last push:").bold(),
            ui::format_relative_time(&repo.pushed_at)
        );
    }

    println!();
    println!("  {} {}", style("Sync status:").bold(), project.sync_status);
    println!();

    Ok(())
}
