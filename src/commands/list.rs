use anyhow::Result;
use indicatif::{ProgressBar, ProgressStyle};
use std::time::Duration;
use tracing::debug;

use crate::cache;
use crate::config::Config;
use crate::discovery;
use crate::project::SyncStatus;
use crate::ui;

pub async fn execute(
    remote: bool,
    local: bool,
    all: bool,
    sort: &str,
    refresh: bool,
) -> Result<()> {
    debug!(remote = %remote, local = %local, all = %all, sort = %sort, "Listing projects");
    let config = Config::load()?;
    let projects_dir = config.projects_dir_expanded();

    // If refresh flag is set, invalidate cache before discovery
    if refresh {
        cache::invalidate_cache()?;
    }

    // Show spinner during discovery
    let spinner = ProgressBar::new_spinner();
    spinner.set_style(
        ProgressStyle::default_spinner()
            .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏")
            .template("{spinner:.cyan} {msg}")
            .unwrap(),
    );
    spinner.set_message("Discovering projects...");
    spinner.enable_steady_tick(Duration::from_millis(80));

    let local_projects = if !remote {
        discovery::discover_local(&projects_dir).await?
    } else {
        Vec::new()
    };

    let remote_projects = if !local {
        discovery::discover_remote(&config).await?
    } else {
        Vec::new()
    };

    let mut projects = discovery::merge_projects(local_projects, remote_projects).await;

    spinner.finish_and_clear();

    // Sort projects based on the selected sort method
    match sort {
        "name" => {
            projects.sort_by(|a, b| a.name.cmp(&b.name));
        }
        "push" => {
            projects.sort_by(|a, b| {
                match (&a.github_repo, &b.github_repo) {
                    (Some(repo_a), Some(repo_b)) => {
                        // Most recently pushed first (reverse chronological)
                        repo_b.pushed_at.cmp(&repo_a.pushed_at)
                    }
                    (Some(_), None) => std::cmp::Ordering::Less,
                    (None, Some(_)) => std::cmp::Ordering::Greater,
                    (None, None) => a.name.cmp(&b.name),
                }
            });
        }
        "dirty" => {
            projects.sort_by(|a, b| {
                let dirty_a = a.git_status.as_ref().map(|s| s.dirty_files).unwrap_or(0);
                let dirty_b = b.git_status.as_ref().map(|s| s.dirty_files).unwrap_or(0);
                // Most dirty first (descending)
                dirty_b.cmp(&dirty_a)
            });
        }
        "status" => {
            projects.sort_by(|a, b| {
                let rank_a = status_rank(&a.sync_status);
                let rank_b = status_rank(&b.sync_status);
                rank_a.cmp(&rank_b)
            });
        }
        _ => {
            anyhow::bail!(
                "Unknown sort method: {}. Use: name, push, dirty, or status",
                sort
            );
        }
    }

    let cache_status = if refresh {
        Some("refreshed".to_string())
    } else {
        cache::cache_age()
    };

    ui::print_project_table(&projects, all || remote, cache_status.as_deref());

    Ok(())
}

// Helper function to rank sync status for sorting
// Lower rank = higher priority (shown first)
fn status_rank(status: &SyncStatus) -> u8 {
    match status {
        SyncStatus::LocalAhead(_) => 0,   // ahead first
        SyncStatus::Diverged => 1,        // then diverged
        SyncStatus::RemoteBehind(_) => 2, // then behind
        SyncStatus::Synced => 3,          // then synced
        SyncStatus::NoGit => 4,           // then no-git
        SyncStatus::LocalOnly => 5,       // then local-only
        SyncStatus::RemoteOnly => 6,      // then remote-only
    }
}
