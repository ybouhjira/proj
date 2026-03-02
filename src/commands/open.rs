use anyhow::{Context, Result};
use console::style;
use dialoguer::{theme::ColorfulTheme, Confirm, FuzzySelect};
use indicatif::{ProgressBar, ProgressStyle};
use std::time::Duration;
use tokio::process::Command;
use tracing::{debug, info};

use crate::config::Config;
use crate::discovery;
use crate::fuzzy;
use crate::project::SyncStatus;

pub async fn execute(name: Option<&str>, github: bool, dir: bool) -> Result<()> {
    info!(name = ?name, "Opening project");
    let config = Config::load()?;
    let projects_dir = config.projects_dir_expanded();

    // Show spinner during project discovery
    let spinner = ProgressBar::new_spinner();
    spinner.set_style(
        ProgressStyle::default_spinner()
            .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏")
            .template("{spinner:.cyan} {msg}")
            .unwrap(),
    );
    spinner.set_message("Discovering projects...");
    spinner.enable_steady_tick(Duration::from_millis(80));

    let local_projects = discovery::discover_local(&projects_dir).await?;
    let remote_projects = discovery::discover_remote(&config).await?;
    let projects = discovery::merge_projects(local_projects, remote_projects).await;

    spinner.finish_and_clear();

    if projects.is_empty() {
        anyhow::bail!("No projects found. Use 'proj new' to create one.");
    }

    // Resolve project name - either from arg or interactive picker
    let selected_project_name = if let Some(query) = name {
        // Fuzzy match against all projects
        let project_names: Vec<String> = projects.iter().map(|p| p.name.clone()).collect();
        let matches = fuzzy::fuzzy_match(query, &project_names);

        debug!(matches = matches.len(), "Fuzzy match results");
        if matches.is_empty() {
            anyhow::bail!("No projects match '{}'", query);
        }

        // If multiple ambiguous matches, show picker
        if matches.len() > 1 && matches[0].1 <= matches[1].1 * 2 {
            let display_items: Vec<String> = matches
                .iter()
                .map(|(name, _)| {
                    let proj = projects.iter().find(|p| &p.name == name).unwrap();
                    format_picker_item(proj)
                })
                .collect();

            let selection = FuzzySelect::with_theme(&ColorfulTheme::default())
                .with_prompt("Multiple matches — select project")
                .items(&display_items)
                .default(0)
                .highlight_matches(true)
                .interact_opt()?;

            match selection {
                Some(idx) => matches[idx].0.clone(),
                None => std::process::exit(0),
            }
        } else {
            matches[0].0.clone()
        }
    } else {
        // No query - show full interactive picker
        println!();
        println!(
            "  {} {}",
            style("◆").cyan().bold(),
            style("Select a project to open in Claude Code").bold()
        );
        println!();

        let display_items: Vec<String> =
            projects.iter().map(format_picker_item).collect();

        let selection = FuzzySelect::with_theme(&ColorfulTheme::default())
            .with_prompt("Search projects")
            .items(&display_items)
            .default(0)
            .highlight_matches(true)
            .interact_opt()?;

        match selection {
            Some(idx) => projects[idx].name.clone(),
            None => std::process::exit(0),
        }
    };

    let project = projects
        .iter()
        .find(|p| p.name == selected_project_name)
        .context("Project not found")?;

    // Handle --github flag
    if github {
        if let Some(ref repo) = project.github_repo {
            println!(
                "\n  {} Opening {} in browser...\n",
                style("🌐").bold(),
                style(&repo.url).underlined().cyan()
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
            anyhow::bail!(
                "Project '{}' has no GitHub repository",
                selected_project_name
            );
        }
    }

    // Handle --dir flag
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
                "\n  {} Opened {} in file manager\n",
                style("📁").bold(),
                style(path.display()).cyan()
            );

            return Ok(());
        } else {
            anyhow::bail!(
                "Project '{}' is not cloned locally",
                selected_project_name
            );
        }
    }

    // Default: Open in Claude Code
    if let Some(ref path) = project.local_path {
        info!(path = %path.display(), "Launching Claude Code");
        crate::ui::launch_claude(path)?;
    } else {
        // Remote-only project - offer to clone first
        println!(
            "\n  {} Project {} is remote-only",
            style("☁️").bold(),
            style(&selected_project_name).cyan().bold()
        );

        let should_clone = Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("Clone it first?")
            .default(true)
            .interact()?;

        if should_clone {
            let clone_path = projects_dir.join(&selected_project_name);

            let clone_spinner = ProgressBar::new_spinner();
            clone_spinner.set_style(
                ProgressStyle::default_spinner()
                    .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏")
                    .template("{spinner:.cyan} {msg}")
                    .unwrap(),
            );
            clone_spinner.set_message(format!("Cloning {}...", &selected_project_name));
            clone_spinner.enable_steady_tick(Duration::from_millis(80));

            let clone_url = format!(
                "git@github.com:{}/{}.git",
                config.github_user, selected_project_name
            );
            let clone_result = Command::new("git")
                .args(["clone", &clone_url, &clone_path.to_string_lossy()])
                .output()
                .await
                .context("Failed to clone repository")?;

            clone_spinner.finish_and_clear();

            if !clone_result.status.success() {
                let stderr = String::from_utf8_lossy(&clone_result.stderr);
                anyhow::bail!("Clone failed: {}", stderr);
            }

            println!(
                "  {} Cloned to {}\n",
                style("✓").green().bold(),
                style(clone_path.display()).cyan()
            );

            crate::ui::launch_claude(&clone_path)?;
        }
    }

    Ok(())
}

fn format_picker_item(project: &crate::project::Project) -> String {
    let status = match &project.sync_status {
        SyncStatus::Synced => style("✓ synced").green().to_string(),
        SyncStatus::LocalAhead(n) => style(format!("⬆ +{}", n)).yellow().to_string(),
        SyncStatus::RemoteBehind(n) => style(format!("⬇ -{}", n)).yellow().to_string(),
        SyncStatus::Diverged => style("⚠ diverged").red().to_string(),
        SyncStatus::LocalOnly => style("● local").cyan().to_string(),
        SyncStatus::RemoteOnly => style("☁ remote").magenta().to_string(),
        SyncStatus::NoGit => style("○ no-git").dim().to_string(),
    };

    let lang = project
        .github_repo
        .as_ref()
        .and_then(|r| r.language.as_ref())
        .map(|l| style(l).dim().to_string())
        .unwrap_or_default();

    let dirty = project
        .git_status
        .as_ref()
        .map(|s| {
            if s.dirty_files > 0 {
                format!(" {}∆", style(s.dirty_files).yellow())
            } else {
                String::new()
            }
        })
        .unwrap_or_default();

    // Pad the name to align columns
    let name_padded = format!("{:<28}", project.name);

    format!("{} {} {}{}", name_padded, status, lang, dirty)
}
