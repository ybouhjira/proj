use anyhow::Result;
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use std::path::Path;
use tokio::process::Command;
use tracing::{debug, info};

use crate::cache;
use crate::config::Config;
use crate::github;
use crate::project::{GitStatus, Project, SyncStatus};

pub async fn discover_local(projects_dir: &Path) -> Result<Vec<Project>> {
    info!(dir = %projects_dir.display(), "Discovering local projects");
    if !projects_dir.exists() {
        return Ok(Vec::new());
    }

    let mut projects = Vec::new();
    let mut entries = tokio::fs::read_dir(projects_dir).await?;

    while let Some(entry) = entries.next_entry().await? {
        let path = entry.path();
        if !path.is_dir() {
            continue;
        }

        let name = entry.file_name().to_string_lossy().to_string();
        let git_dir = path.join(".git");

        if git_dir.exists() {
            let git_status = get_git_status(&path).await.ok();
            projects.push(Project {
                name,
                local_path: Some(path),
                github_repo: None,
                sync_status: SyncStatus::LocalOnly,
                git_status,
            });
        } else {
            projects.push(Project {
                name,
                local_path: Some(path),
                github_repo: None,
                sync_status: SyncStatus::NoGit,
                git_status: None,
            });
        }
    }

    debug!(count = projects.len(), "Local projects found");
    Ok(projects)
}

pub async fn discover_remote(config: &Config) -> Result<Vec<Project>> {
    info!("Discovering remote projects");
    // Try to get cached repos first
    let repos = if let Some(cached) = cache::get_cached_repos() {
        cached
    } else {
        // Cache miss - fetch from GitHub API
        let repos = github::list_repos(&config.github_user).await?;

        // Save to cache
        if let Err(e) = cache::save_cache(&repos) {
            eprintln!("Warning: Failed to save cache: {}", e);
        }

        repos
    };

    debug!(count = repos.len(), "Remote repos found");
    Ok(repos
        .into_iter()
        .map(|repo| {
            let name = repo
                .full_name
                .split('/')
                .next_back()
                .unwrap_or(&repo.full_name)
                .to_string();
            Project {
                name,
                local_path: None,
                github_repo: Some(repo),
                sync_status: SyncStatus::RemoteOnly,
                git_status: None,
            }
        })
        .collect())
}

pub async fn merge_projects(local: Vec<Project>, remote: Vec<Project>) -> Vec<Project> {
    debug!(local = local.len(), remote = remote.len(), "Merging projects");
    let mut merged: HashMap<String, Project> = HashMap::new();

    for mut project in local {
        if let Some(ref git_status) = project.git_status {
            if let Some(local_path) = &project.local_path {
                project.sync_status = determine_sync_status(local_path, git_status)
                    .await
                    .unwrap_or(SyncStatus::LocalOnly);
            }
        }
        merged.insert(project.name.clone(), project);
    }

    for remote_project in remote {
        if let Some(local_project) = merged.get_mut(&remote_project.name) {
            local_project.github_repo = remote_project.github_repo;
            if local_project.sync_status == SyncStatus::LocalOnly {
                if let (Some(ref local_path), Some(ref git_status)) =
                    (&local_project.local_path, &local_project.git_status)
                {
                    local_project.sync_status = determine_sync_status(local_path, git_status)
                        .await
                        .unwrap_or(SyncStatus::LocalOnly);
                }
            }
        } else {
            merged.insert(remote_project.name.clone(), remote_project);
        }
    }

    let mut projects: Vec<Project> = merged.into_values().collect();
    projects.sort_by(|a, b| a.name.cmp(&b.name));
    projects
}

async fn get_git_status(path: &Path) -> Result<GitStatus> {
    let branch_output = Command::new("git")
        .args(["rev-parse", "--abbrev-ref", "HEAD"])
        .current_dir(path)
        .output()
        .await?;

    let branch = String::from_utf8_lossy(&branch_output.stdout)
        .trim()
        .to_string();

    let status_output = Command::new("git")
        .args(["status", "--porcelain"])
        .current_dir(path)
        .output()
        .await?;

    let dirty_files = String::from_utf8_lossy(&status_output.stdout)
        .lines()
        .count() as u32;

    let log_output = Command::new("git")
        .args(["log", "-1", "--format=%s%n%ct"])
        .current_dir(path)
        .output()
        .await?;

    let log_output_str = String::from_utf8_lossy(&log_output.stdout);
    let log_lines: Vec<&str> = log_output_str.lines().collect();

    let last_commit_msg = log_lines
        .first()
        .map(|s| s.to_string())
        .unwrap_or_else(|| "No commits".to_string());

    let last_commit_date = if log_lines.len() > 1 {
        log_lines[1]
            .parse::<i64>()
            .ok()
            .and_then(|ts| DateTime::from_timestamp(ts, 0))
            .unwrap_or_else(Utc::now)
    } else {
        Utc::now()
    };

    Ok(GitStatus {
        branch,
        dirty_files,
        last_commit_msg,
        last_commit_date,
    })
}

async fn determine_sync_status(path: &Path, _git_status: &GitStatus) -> Result<SyncStatus> {
    let remote_output = Command::new("git")
        .args(["remote", "-v"])
        .current_dir(path)
        .output()
        .await?;

    if remote_output.stdout.is_empty() {
        return Ok(SyncStatus::LocalOnly);
    }

    let fetch_output = Command::new("git")
        .args(["fetch", "--dry-run"])
        .current_dir(path)
        .output()
        .await;

    if fetch_output.is_err() {
        return Ok(SyncStatus::Synced);
    }

    let rev_list_output = Command::new("git")
        .args(["rev-list", "--left-right", "--count", "HEAD...@{upstream}"])
        .current_dir(path)
        .output()
        .await;

    if let Ok(output) = rev_list_output {
        if output.status.success() {
            let counts = String::from_utf8_lossy(&output.stdout);
            let parts: Vec<&str> = counts.split_whitespace().collect();

            if parts.len() == 2 {
                let ahead: u32 = parts[0].parse().unwrap_or(0);
                let behind: u32 = parts[1].parse().unwrap_or(0);

                return Ok(match (ahead, behind) {
                    (0, 0) => SyncStatus::Synced,
                    (a, 0) if a > 0 => SyncStatus::LocalAhead(a),
                    (0, b) if b > 0 => SyncStatus::RemoteBehind(b),
                    (_, _) => SyncStatus::Diverged,
                });
            }
        }
    }

    Ok(SyncStatus::Synced)
}
