use anyhow::Result;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

/// Event types from file system watching
#[derive(Debug, Clone, PartialEq)]
pub enum WatchEvent {
    FileCreated(PathBuf),
    FileModified(PathBuf),
    FileDeleted(PathBuf),
    GitCommit { message: String, hash: String },
    SyncStatusChanged,
}

/// Configuration for the watcher
#[derive(Debug, Clone)]
pub struct WatchConfig {
    /// How often to poll for changes (in ms)
    pub poll_interval: Duration,
    /// Paths/patterns to ignore
    pub ignore_patterns: Vec<String>,
    /// Whether to show desktop notifications
    pub notify: bool,
}

impl Default for WatchConfig {
    fn default() -> Self {
        Self {
            poll_interval: Duration::from_secs(2),
            ignore_patterns: vec![
                "target".to_string(),
                "node_modules".to_string(),
                ".git".to_string(),
                "dist".to_string(),
                "build".to_string(),
            ],
            notify: false,
        }
    }
}

/// Snapshot of project status at a point in time
#[derive(Debug, Clone)]
pub struct WatchSnapshot {
    pub dirty_files: u32,
    pub branch: String,
    pub last_commit: String,
    pub timestamp: Instant,
}

/// Check if a path should be ignored based on patterns
pub fn should_ignore(path: &Path, patterns: &[String]) -> bool {
    for pattern in patterns {
        if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
            if name == pattern {
                return true;
            }
        }
        // Also check if any ancestor matches
        for ancestor in path.ancestors() {
            if let Some(name) = ancestor.file_name().and_then(|n| n.to_str()) {
                if name == pattern {
                    return true;
                }
            }
        }
    }
    false
}

/// Diff two snapshots to produce events
pub fn diff_snapshots(old: &WatchSnapshot, new: &WatchSnapshot) -> Vec<WatchEvent> {
    todo!("Compare snapshots and return events")
}

/// Take a snapshot of current project state
pub async fn take_snapshot(project_path: &Path) -> Result<WatchSnapshot> {
    todo!("Get current git status snapshot")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_watch_config_default() {
        let config = WatchConfig::default();
        assert_eq!(config.poll_interval, Duration::from_secs(2));
        assert_eq!(config.notify, false);
        assert!(!config.ignore_patterns.is_empty());
    }

    #[test]
    fn test_watch_config_default_ignore_patterns() {
        let config = WatchConfig::default();
        assert!(config.ignore_patterns.contains(&"target".to_string()));
        assert!(config.ignore_patterns.contains(&"node_modules".to_string()));
        assert!(config.ignore_patterns.contains(&".git".to_string()));
    }

    #[test]
    fn test_should_ignore_target() {
        let patterns = vec!["target".to_string(), "node_modules".to_string()];
        let path = PathBuf::from("project/target/debug/main");
        assert!(should_ignore(&path, &patterns));
    }

    #[test]
    fn test_should_ignore_node_modules() {
        let patterns = vec!["target".to_string(), "node_modules".to_string()];
        let path = PathBuf::from("project/node_modules/package");
        assert!(should_ignore(&path, &patterns));
    }

    #[test]
    fn test_should_ignore_git() {
        let patterns = vec![".git".to_string()];
        let path = PathBuf::from("project/.git/objects");
        assert!(should_ignore(&path, &patterns));
    }

    #[test]
    fn test_should_ignore_nested_path() {
        let patterns = vec!["target".to_string()];
        let path = PathBuf::from("src/foo/target/debug/main");
        assert!(should_ignore(&path, &patterns));
    }

    #[test]
    fn test_should_not_ignore_regular_file() {
        let patterns = vec!["target".to_string(), "node_modules".to_string()];
        let path = PathBuf::from("src/main.rs");
        assert!(!should_ignore(&path, &patterns));
    }

    #[test]
    fn test_should_not_ignore_empty_patterns() {
        let patterns = vec![];
        let path = PathBuf::from("src/target/main.rs");
        assert!(!should_ignore(&path, &patterns));
    }

    #[test]
    fn test_watch_event_equality() {
        let event1 = WatchEvent::FileCreated(PathBuf::from("test.rs"));
        let event2 = WatchEvent::FileCreated(PathBuf::from("test.rs"));
        let event3 = WatchEvent::FileCreated(PathBuf::from("other.rs"));

        assert_eq!(event1, event2);
        assert_ne!(event1, event3);
    }

    #[test]
    fn test_watch_event_variants() {
        let created = WatchEvent::FileCreated(PathBuf::from("file.rs"));
        let modified = WatchEvent::FileModified(PathBuf::from("file.rs"));
        let deleted = WatchEvent::FileDeleted(PathBuf::from("file.rs"));
        let commit = WatchEvent::GitCommit {
            message: "Initial commit".to_string(),
            hash: "abc123".to_string(),
        };
        let sync = WatchEvent::SyncStatusChanged;

        assert_ne!(created, modified);
        assert_ne!(modified, deleted);
        assert_ne!(deleted, commit);
        assert_ne!(commit, sync);
    }

    #[test]
    #[should_panic(expected = "not yet implemented")]
    fn test_diff_snapshots_stub() {
        let snapshot1 = WatchSnapshot {
            dirty_files: 0,
            branch: "main".to_string(),
            last_commit: "abc123".to_string(),
            timestamp: Instant::now(),
        };
        let snapshot2 = WatchSnapshot {
            dirty_files: 1,
            branch: "main".to_string(),
            last_commit: "abc123".to_string(),
            timestamp: Instant::now(),
        };

        diff_snapshots(&snapshot1, &snapshot2);
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented")]
    async fn test_take_snapshot_stub() {
        let path = PathBuf::from("/tmp/test");
        take_snapshot(&path).await.unwrap();
    }
}
