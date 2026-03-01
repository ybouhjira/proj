use chrono::{DateTime, Utc};
use std::fmt;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Project {
    pub name: String,
    pub local_path: Option<PathBuf>,
    pub github_repo: Option<GitHubRepo>,
    pub sync_status: SyncStatus,
    pub git_status: Option<GitStatus>,
}

#[derive(Debug, Clone)]
pub struct GitHubRepo {
    pub full_name: String,
    pub description: Option<String>,
    pub is_private: bool,
    pub default_branch: String,
    pub pushed_at: DateTime<Utc>,
    pub language: Option<String>,
    pub stars: u32,
    pub url: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SyncStatus {
    Synced,
    LocalAhead(u32),
    RemoteBehind(u32),
    Diverged,
    LocalOnly,
    RemoteOnly,
    NoGit,
}

impl fmt::Display for SyncStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let emoji = match self {
            SyncStatus::Synced => "✅",
            SyncStatus::LocalAhead(_) => "⬆",
            SyncStatus::RemoteBehind(_) => "⬇",
            SyncStatus::Diverged => "⚠",
            SyncStatus::LocalOnly => "💻",
            SyncStatus::RemoteOnly => "☁️",
            SyncStatus::NoGit => "📁",
        };
        write!(f, "{}", emoji)
    }
}

#[derive(Debug, Clone)]
pub struct GitStatus {
    pub branch: String,
    pub dirty_files: u32,
    pub last_commit_msg: String,
    pub last_commit_date: DateTime<Utc>,
}

#[allow(dead_code)]
pub trait QualityCheck {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn run(&self, project_path: &std::path::Path) -> CheckResult;
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct CheckResult {
    pub score: f32,
    pub issues: Vec<Issue>,
    pub suggestions: Vec<String>,
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Issue {
    pub severity: Severity,
    pub message: String,
    pub file: Option<String>,
    pub line: Option<u32>,
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum Severity {
    Error,
    Warning,
    Info,
}
