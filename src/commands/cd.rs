use anyhow::{Context, Result};
use console::style;
use dialoguer::{theme::ColorfulTheme, Select};

use crate::config::Config;
use crate::discovery;
use crate::fuzzy;

pub async fn execute(query: Option<&str>) -> Result<()> {
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

    // If no query provided, show interactive picker with all projects
    if query.is_none() {
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select project")
            .items(&local_names)
            .default(0)
            .interact_opt()?;

        if let Some(index) = selection {
            let project_name = &local_names[index];
            let project = projects
                .iter()
                .find(|p| &p.name == project_name)
                .context("Project not found")?;

            if let Some(path) = &project.local_path {
                println!("{}", path.display());
                return Ok(());
            }
        } else {
            // User cancelled with Esc
            std::process::exit(1);
        }
    }

    let query = query.unwrap();
    let matches = fuzzy::fuzzy_match(query, &local_names);

    if matches.is_empty() {
        eprintln!("{} No projects match '{}'", style("✗").red(), query);
        std::process::exit(1);
    }

    // If exact match or clear winner, use it
    if matches.len() == 1 || (matches.len() > 1 && matches[0].1 > matches[1].1 * 2) {
        let project_name = &matches[0].0;
        let project = projects
            .iter()
            .find(|p| &p.name == project_name)
            .context("Project not found")?;

        if let Some(path) = &project.local_path {
            println!("{}", path.display());
            return Ok(());
        }
    }

    // Multiple ambiguous matches - show interactive picker
    let match_names: Vec<String> = matches.iter().map(|(name, _)| name.clone()).collect();

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select project")
        .items(&match_names)
        .default(0)
        .interact_opt()?;

    if let Some(index) = selection {
        let project_name = &match_names[index];
        let project = projects
            .iter()
            .find(|p| &p.name == project_name)
            .context("Project not found")?;

        if let Some(path) = &project.local_path {
            println!("{}", path.display());
            return Ok(());
        }
    } else {
        // User cancelled with Esc
        std::process::exit(1);
    }

    Ok(())
}
