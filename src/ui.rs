use chrono::{DateTime, Duration, Utc};
use console::{style, Style};
use std::collections::HashMap;
use tabled::{
    builder::Builder,
    settings::{object::Rows, Alignment, Modify, Style as TableStyle},
};

use crate::project::{Project, SyncStatus};

pub fn print_project_table(projects: &[Project], show_remote: bool) {
    let local_count = projects.iter().filter(|p| p.local_path.is_some()).count();
    let remote_count = projects.iter().filter(|p| p.github_repo.is_some()).count();

    println!();
    println!("  {} Projects ({} local · {} remote)",
        style("📦").bold(),
        style(local_count).cyan(),
        style(remote_count).dim()
    );
    println!();

    let visible_projects: Vec<&Project> = if show_remote {
        projects.iter().collect()
    } else {
        projects.iter().filter(|p| p.local_path.is_some()).collect()
    };

    if visible_projects.is_empty() {
        println!("  {}", style("No projects found").dim());
        return;
    }

    let mut builder = Builder::default();
    builder.push_record(vec!["NAME", "STATUS", "BRANCH", "DIRTY", "LAST PUSH"]);

    for project in &visible_projects {
        let name = if project.local_path.is_some() {
            style(&project.name).bold().to_string()
        } else {
            style(&project.name).dim().to_string()
        };

        let status = format_status(&project.sync_status);

        let branch = if let Some(ref git_status) = project.git_status {
            style(&git_status.branch).dim().to_string()
        } else {
            style("—").dim().to_string()
        };

        let dirty = if let Some(ref git_status) = project.git_status {
            if git_status.dirty_files > 0 {
                style(format!("{}∆", git_status.dirty_files)).yellow().to_string()
            } else {
                style("0∆").dim().to_string()
            }
        } else {
            style("—").dim().to_string()
        };

        let last_push = if let Some(ref repo) = project.github_repo {
            format_relative_time(&repo.pushed_at)
        } else {
            style("—").dim().to_string()
        };

        builder.push_record(vec![name, status, branch, dirty, last_push]);
    }

    let mut table = builder.build();
    table
        .with(TableStyle::blank())
        .with(Modify::new(Rows::first()).with(Alignment::left()));

    println!("{}", table);

    let remote_only_count = projects.iter().filter(|p| p.sync_status == SyncStatus::RemoteOnly).count();
    if remote_only_count > 0 && !show_remote {
        println!();
        println!("  {} Remote only ({}): {} to see all",
            style("☁️").dim(),
            style(remote_only_count).dim(),
            style("proj ls --remote").cyan()
        );
    }
    println!();
}

pub fn print_sync_dashboard(projects: &[Project]) {
    println!();
    println!("  {} Sync Dashboard", style("🔄").bold());
    println!();

    let mut groups: HashMap<&str, Vec<&Project>> = HashMap::new();

    for project in projects {
        if project.local_path.is_none() {
            continue;
        }

        match &project.sync_status {
            SyncStatus::LocalAhead(_) => groups.entry("⬆ Need push").or_default().push(project),
            SyncStatus::RemoteBehind(_) => groups.entry("⬇ Need pull").or_default().push(project),
            SyncStatus::Diverged => groups.entry("⚠ Diverged").or_default().push(project),
            SyncStatus::NoGit => groups.entry("💻 No git").or_default().push(project),
            SyncStatus::Synced => {
                if project.git_status.as_ref().map_or(false, |s| s.dirty_files > 0) {
                    groups.entry("⚠ Dirty").or_default().push(project);
                } else {
                    groups.entry("✅ Clean").or_default().push(project);
                }
            }
            _ => {}
        }
    }

    let order = ["⬆ Need push", "⬇ Need pull", "⚠ Diverged", "⚠ Dirty", "💻 No git", "✅ Clean"];

    for category in order {
        if let Some(projects) = groups.get(category) {
            println!("  {} ({}):", style(category).bold(), projects.len());
            for project in projects.iter().take(10) {
                let detail = match &project.sync_status {
                    SyncStatus::LocalAhead(n) => format!("  +{} commit{}", n, if *n > 1 { "s" } else { "" }),
                    SyncStatus::RemoteBehind(n) => format!("  -{} commit{}", n, if *n > 1 { "s" } else { "" }),
                    _ => {
                        if let Some(ref git_status) = project.git_status {
                            if git_status.dirty_files > 0 {
                                format!("  {} file{}",
                                    git_status.dirty_files,
                                    if git_status.dirty_files > 1 { "s" } else { "" }
                                )
                            } else {
                                String::new()
                            }
                        } else {
                            String::new()
                        }
                    }
                };

                let branch = project.git_status.as_ref()
                    .map(|s| format!(" {}", style(&s.branch).dim()))
                    .unwrap_or_default();

                println!("    {}{}{}",
                    style(&project.name).cyan(),
                    branch,
                    style(detail).yellow()
                );
            }

            if projects.len() > 10 {
                println!("    {} ... and {} more",
                    style("").dim(),
                    style(projects.len() - 10).dim()
                );
            }
            println!();
        }
    }
}

pub fn format_relative_time(dt: &DateTime<Utc>) -> String {
    let now = Utc::now();
    let diff = now.signed_duration_since(*dt);

    if diff < Duration::zero() {
        return style("just now").dim().to_string();
    }

    let style_time = Style::new().dim();

    if diff.num_seconds() < 60 {
        style_time.apply_to("just now").to_string()
    } else if diff.num_minutes() < 60 {
        style_time.apply_to(format!("{}m ago", diff.num_minutes())).to_string()
    } else if diff.num_hours() < 24 {
        style_time.apply_to(format!("{}h ago", diff.num_hours())).to_string()
    } else if diff.num_days() < 7 {
        style_time.apply_to(format!("{}d ago", diff.num_days())).to_string()
    } else if diff.num_weeks() < 4 {
        style_time.apply_to(format!("{}w ago", diff.num_weeks())).to_string()
    } else if diff.num_days() < 365 {
        style_time.apply_to(format!("{}mo ago", diff.num_days() / 30)).to_string()
    } else {
        style_time.apply_to(format!("{}y ago", diff.num_days() / 365)).to_string()
    }
}

fn format_status(status: &SyncStatus) -> String {
    match status {
        SyncStatus::Synced => {
            format!("{} {}", "✅", style("synced").green())
        }
        SyncStatus::LocalAhead(n) => {
            format!("{} {}", "⬆", style(format!("ahead {}", n)).yellow())
        }
        SyncStatus::RemoteBehind(n) => {
            format!("{} {}", "⬇", style(format!("behind {}", n)).yellow())
        }
        SyncStatus::Diverged => {
            format!("{} {}", "⚠", style("diverged").red())
        }
        SyncStatus::LocalOnly => {
            format!("{} {}", "💻", style("local").cyan())
        }
        SyncStatus::RemoteOnly => {
            format!("{} {}", "☁️", style("remote").dim())
        }
        SyncStatus::NoGit => {
            format!("{} {}", "📁", style("no-git").dim())
        }
    }
}
