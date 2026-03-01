use anyhow::{Context, Result};
use console::style;
use tokio::process::Command;

use crate::checks;
use crate::config::Config;
use crate::discovery;
use crate::fuzzy;

pub async fn execute(name: &str, all: bool, check: Option<String>, use_ai: bool) -> Result<()> {
    // Check if AI flag is used
    if use_ai {
        let claude_available = Command::new("claude")
            .arg("--version")
            .output()
            .await
            .is_ok();

        if !claude_available {
            println!();
            println!(
                "  {} AI analysis requires '{}' CLI",
                style("⚠").yellow(),
                style("claude").cyan()
            );
            println!(
                "  Install: {}",
                style("npm install -g @anthropic-ai/claude-cli").dim()
            );
            println!();
            return Ok(());
        }
    }

    // Resolve project
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

    if project.local_path.is_none() {
        anyhow::bail!(
            "Project '{}' is not cloned locally. Use 'proj clone {}' first.",
            project_name,
            project_name
        );
    }

    let project_path = project.local_path.as_ref().unwrap();

    // Run checks
    let report = checks::run_checks(project_path, check.as_deref(), all, use_ai).await?;

    // Display report
    display_report(&report);

    Ok(())
}

fn display_report(report: &checks::runner::CheckReport) {
    println!();
    println!(
        "  {} Code Quality Report: {}",
        style("🔍").bold(),
        style(&report.project_name).cyan().bold()
    );
    println!("  Language: {}", style(&report.lang).dim());
    println!();

    // Overall score bar
    let overall_percent = (report.overall_score * 100.0) as u32;
    let bar = create_progress_bar(report.overall_score);

    println!(
        "  {}  {}  {}",
        style("OVERALL").bold(),
        bar,
        format_score_percent(overall_percent, report.overall_score)
    );
    println!();

    // Individual checks
    println!(
        "  {}      {}  {}",
        style("CHECK").bold(),
        style("SCORE").bold(),
        style("ISSUES").bold()
    );

    for (check_name, result) in &report.checks {
        let score_percent = (result.score * 100.0) as u32;
        let bar = create_progress_bar(result.score);

        let status = if result.score > 0.8 {
            format!(
                "{} {}",
                style("✅").green(),
                get_check_message(check_name, result)
            )
        } else if result.score > 0.5 {
            format!(
                "{} {}",
                style("⚠").yellow(),
                get_check_message(check_name, result)
            )
        } else {
            format!(
                "{} {}",
                style("❌").red(),
                get_check_message(check_name, result)
            )
        };

        println!(
            "  {:8}   {}  {}  {}",
            style(check_name).cyan(),
            bar,
            format_score_percent(score_percent, result.score),
            status
        );
    }

    // Suggestions
    let all_suggestions: Vec<&String> = report
        .checks
        .iter()
        .flat_map(|(_, result)| &result.suggestions)
        .collect();

    if !all_suggestions.is_empty() {
        println!();
        println!("  {} Suggestions:", style("💡").bold());
        for suggestion in all_suggestions {
            println!("    {} {}", style("•").dim(), suggestion);
        }
    }

    println!();
}

fn create_progress_bar(score: f32) -> String {
    let filled = (score * 10.0) as usize;
    let empty = 10 - filled;

    let filled_str = style("█").green().to_string().repeat(filled);
    let empty_str = style("░").dim().to_string().repeat(empty);

    format!("{}{}", filled_str, empty_str)
}

fn format_score_percent(percent: u32, score: f32) -> String {
    let text = format!("{:>3}%", percent);
    if score > 0.8 {
        style(text).green().to_string()
    } else if score > 0.5 {
        style(text).yellow().to_string()
    } else {
        style(text).red().to_string()
    }
}

fn get_check_message(check_name: &str, result: &crate::project::CheckResult) -> String {
    if result.suggestions.is_empty() {
        match check_name {
            "quality" => "No issues found".to_string(),
            "testing" => "Good test coverage".to_string(),
            "logging" => "Structured logging in place".to_string(),
            "security" => "No vulnerabilities detected".to_string(),
            "docs" => "Well documented".to_string(),
            _ => "Passed".to_string(),
        }
    } else {
        result.suggestions.first().unwrap_or(&String::new()).clone()
    }
}
