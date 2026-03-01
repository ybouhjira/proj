use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use serde::Deserialize;
use tokio::process::Command;

use crate::project::GitHubRepo;

#[derive(Debug, Deserialize)]
struct GhRepo {
    name: String,
    #[serde(rename = "nameWithOwner")]
    full_name: String,
    description: Option<String>,
    #[serde(rename = "isPrivate")]
    is_private: bool,
    #[serde(rename = "defaultBranchRef")]
    default_branch_ref: Option<DefaultBranch>,
    #[serde(rename = "pushedAt")]
    pushed_at: DateTime<Utc>,
    #[serde(rename = "primaryLanguage")]
    primary_language: Option<Language>,
    #[serde(rename = "stargazerCount")]
    stargazer_count: u32,
    url: String,
}

#[derive(Debug, Deserialize)]
struct DefaultBranch {
    name: String,
}

#[derive(Debug, Deserialize)]
struct Language {
    name: String,
}

pub async fn check_gh_cli() -> Result<()> {
    which::which("gh").context("gh CLI not found. Install it: https://cli.github.com")?;
    Ok(())
}

pub async fn list_repos(username: &str) -> Result<Vec<GitHubRepo>> {
    let output = Command::new("gh")
        .args([
            "repo",
            "list",
            username,
            "--limit",
            "1000",
            "--json",
            "name,nameWithOwner,description,isPrivate,defaultBranchRef,pushedAt,primaryLanguage,stargazerCount,url",
        ])
        .output()
        .await
        .context("Failed to run gh CLI")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("gh CLI error: {}", stderr);
    }

    let gh_repos: Vec<GhRepo> =
        serde_json::from_slice(&output.stdout).context("Failed to parse gh output")?;

    Ok(gh_repos
        .into_iter()
        .map(|r| GitHubRepo {
            full_name: r.full_name,
            description: r.description,
            is_private: r.is_private,
            default_branch: r
                .default_branch_ref
                .map(|b| b.name)
                .unwrap_or_else(|| "main".to_string()),
            pushed_at: r.pushed_at,
            language: r.primary_language.map(|l| l.name),
            stars: r.stargazer_count,
            url: r.url,
        })
        .collect())
}

pub async fn create_repo(name: &str, private: bool) -> Result<()> {
    let mut args = vec!["repo", "create", name];

    if private {
        args.push("--private");
    } else {
        args.push("--public");
    }

    let output = Command::new("gh")
        .args(&args)
        .output()
        .await
        .context("Failed to create GitHub repo")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("Failed to create repo: {}", stderr);
    }

    Ok(())
}

pub async fn repo_info(name: &str) -> Result<GitHubRepo> {
    let output = Command::new("gh")
        .args([
            "repo",
            "view",
            name,
            "--json",
            "name,nameWithOwner,description,isPrivate,defaultBranchRef,pushedAt,primaryLanguage,stargazerCount,url",
        ])
        .output()
        .await
        .context("Failed to get repo info")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("Failed to get repo info: {}", stderr);
    }

    let gh_repo: GhRepo =
        serde_json::from_slice(&output.stdout).context("Failed to parse repo info")?;

    Ok(GitHubRepo {
        full_name: gh_repo.full_name,
        description: gh_repo.description,
        is_private: gh_repo.is_private,
        default_branch: gh_repo
            .default_branch_ref
            .map(|b| b.name)
            .unwrap_or_else(|| "main".to_string()),
        pushed_at: gh_repo.pushed_at,
        language: gh_repo.primary_language.map(|l| l.name),
        stars: gh_repo.stargazer_count,
        url: gh_repo.url,
    })
}
