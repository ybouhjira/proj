use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
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

#[derive(Debug, Clone, Serialize, Deserialize)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sync_status_equality() {
        assert_eq!(SyncStatus::Synced, SyncStatus::Synced);
        assert_eq!(SyncStatus::LocalAhead(3), SyncStatus::LocalAhead(3));
        assert_ne!(SyncStatus::LocalAhead(3), SyncStatus::LocalAhead(5));
        assert_eq!(SyncStatus::Diverged, SyncStatus::Diverged);
        assert_ne!(SyncStatus::Synced, SyncStatus::Diverged);
    }

    #[test]
    fn test_sync_status_display() {
        assert_eq!(SyncStatus::Synced.to_string(), "✅");
        assert_eq!(SyncStatus::LocalAhead(0).to_string(), "⬆");
        assert_eq!(SyncStatus::RemoteBehind(0).to_string(), "⬇");
        assert_eq!(SyncStatus::Diverged.to_string(), "⚠");
        assert_eq!(SyncStatus::LocalOnly.to_string(), "💻");
        assert_eq!(SyncStatus::RemoteOnly.to_string(), "☁️");
        assert_eq!(SyncStatus::NoGit.to_string(), "📁");
    }

    #[test]
    fn test_github_repo_creation() {
        let repo = GitHubRepo {
            full_name: "user/repo".to_string(),
            description: Some("Test repository".to_string()),
            is_private: false,
            default_branch: "main".to_string(),
            pushed_at: Utc::now(),
            language: Some("Rust".to_string()),
            stars: 42,
            url: "https://github.com/user/repo".to_string(),
        };

        assert_eq!(repo.full_name, "user/repo");
        assert_eq!(repo.stars, 42);
        assert!(!repo.is_private);
        assert_eq!(repo.language, Some("Rust".to_string()));
    }

    #[test]
    fn test_github_repo_serialization() {
        let repo = GitHubRepo {
            full_name: "test/repo".to_string(),
            description: None,
            is_private: true,
            default_branch: "develop".to_string(),
            pushed_at: Utc::now(),
            language: None,
            stars: 0,
            url: "https://github.com/test/repo".to_string(),
        };

        let json = serde_json::to_string(&repo).unwrap();
        let deserialized: GitHubRepo = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.full_name, "test/repo");
        assert_eq!(deserialized.is_private, true);
        assert_eq!(deserialized.default_branch, "develop");
    }

    #[test]
    fn test_project_with_local_path() {
        let project = Project {
            name: "test-project".to_string(),
            local_path: Some(PathBuf::from("/home/user/projects/test-project")),
            github_repo: None,
            sync_status: SyncStatus::LocalOnly,
            git_status: None,
        };

        assert!(project.local_path.is_some());
        assert!(project.github_repo.is_none());
        assert_eq!(project.sync_status, SyncStatus::LocalOnly);
    }

    #[test]
    fn test_project_with_github_repo() {
        let repo = GitHubRepo {
            full_name: "user/project".to_string(),
            description: Some("A test project".to_string()),
            is_private: false,
            default_branch: "main".to_string(),
            pushed_at: Utc::now(),
            language: Some("TypeScript".to_string()),
            stars: 10,
            url: "https://github.com/user/project".to_string(),
        };

        let project = Project {
            name: "project".to_string(),
            local_path: None,
            github_repo: Some(repo),
            sync_status: SyncStatus::RemoteOnly,
            git_status: None,
        };

        assert!(project.github_repo.is_some());
        assert!(project.local_path.is_none());
        assert_eq!(project.sync_status, SyncStatus::RemoteOnly);
    }

    #[test]
    fn test_git_status_creation() {
        let git_status = GitStatus {
            branch: "feature/test".to_string(),
            dirty_files: 3,
            last_commit_msg: "Add tests".to_string(),
            last_commit_date: Utc::now(),
        };

        assert_eq!(git_status.branch, "feature/test");
        assert_eq!(git_status.dirty_files, 3);
        assert_eq!(git_status.last_commit_msg, "Add tests");
    }

    #[test]
    fn test_sync_status_variants() {
        let statuses = vec![
            SyncStatus::Synced,
            SyncStatus::LocalAhead(1),
            SyncStatus::RemoteBehind(2),
            SyncStatus::Diverged,
            SyncStatus::LocalOnly,
            SyncStatus::RemoteOnly,
            SyncStatus::NoGit,
        ];

        assert_eq!(statuses.len(), 7);
        for status in statuses {
            let display = status.to_string();
            assert!(!display.is_empty());
        }
    }

    #[test]
    fn test_project_clone() {
        let project = Project {
            name: "clone-test".to_string(),
            local_path: Some(PathBuf::from("/test")),
            github_repo: None,
            sync_status: SyncStatus::Synced,
            git_status: None,
        };

        let cloned = project.clone();
        assert_eq!(cloned.name, project.name);
        assert_eq!(cloned.sync_status, project.sync_status);
    }
}
