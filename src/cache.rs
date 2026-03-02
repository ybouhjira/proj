use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use anyhow::Result;
use serde::{Deserialize, Serialize};
use tracing::debug;

use crate::project::GitHubRepo;

const CACHE_TTL_SECS: u64 = 300; // 5 minutes

#[derive(Serialize, Deserialize)]
struct Cache {
    repos: Vec<GitHubRepo>,
    cached_at: u64, // unix timestamp
}

/// Get cached repos or None if cache is stale/missing
pub fn get_cached_repos() -> Option<Vec<GitHubRepo>> {
    debug!("Checking cache");
    let path = cache_path();

    if !path.exists() {
        debug!("No cache file found");
        return None;
    }

    let contents = fs::read_to_string(&path).ok()?;
    let cache: Cache = serde_json::from_str(&contents).ok()?;

    if !is_cache_valid(cache.cached_at) {
        debug!("Cache stale, needs refresh");
        return None;
    }

    debug!(count = cache.repos.len(), "Cache hit");
    Some(cache.repos)
}

/// Save repos to cache
pub fn save_cache(repos: &[GitHubRepo]) -> Result<()> {
    debug!(count = repos.len(), "Saving cache");
    let path = cache_path();

    // Create parent directories if they don't exist
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let cache = Cache {
        repos: repos.to_vec(),
        cached_at: now,
    };

    let json = serde_json::to_string_pretty(&cache)?;
    fs::write(&path, json)?;

    Ok(())
}

/// Get cache file path (~/.config/proj/cache.json)
fn cache_path() -> PathBuf {
    let mut path = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
    path.push("proj");
    path.push("cache.json");
    path
}

/// Check if cache is still valid
fn is_cache_valid(cached_at: u64) -> bool {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    now - cached_at < CACHE_TTL_SECS
}

/// Invalidate cache (delete file)
pub fn invalidate_cache() -> Result<()> {
    debug!("Cache invalidated");
    let path = cache_path();

    if path.exists() {
        fs::remove_file(path)?;
    }

    Ok(())
}

/// Get cache age as human-readable string
pub fn cache_age() -> Option<String> {
    let path = cache_path();

    if !path.exists() {
        return None;
    }

    let contents = fs::read_to_string(&path).ok()?;
    let cache: Cache = serde_json::from_str(&contents).ok()?;

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let age_secs = now - cache.cached_at;

    if age_secs < 60 {
        Some(format!("{}s ago", age_secs))
    } else if age_secs < 3600 {
        Some(format!("{}m ago", age_secs / 60))
    } else if age_secs < 86400 {
        Some(format!("{}h ago", age_secs / 3600))
    } else {
        Some(format!("{}d ago", age_secs / 86400))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[test]
    fn test_cache_validity() {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        assert!(is_cache_valid(now - 100));
        assert!(is_cache_valid(now - CACHE_TTL_SECS + 10));
        assert!(!is_cache_valid(now - CACHE_TTL_SECS - 10));
        assert!(!is_cache_valid(now - 1000));
    }

    #[test]
    fn test_cache_serialization_roundtrip() {
        let repos = vec![
            GitHubRepo {
                full_name: "user/repo1".to_string(),
                description: Some("Desc 1".to_string()),
                is_private: true,
                default_branch: "main".to_string(),
                pushed_at: Utc::now(),
                language: Some("TypeScript".to_string()),
                stars: 100,
                url: "https://github.com/user/repo1".to_string(),
            },
            GitHubRepo {
                full_name: "user/repo2".to_string(),
                description: None,
                is_private: false,
                default_branch: "master".to_string(),
                pushed_at: Utc::now(),
                language: Some("Python".to_string()),
                stars: 50,
                url: "https://github.com/user/repo2".to_string(),
            },
        ];

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let cache = Cache {
            repos: repos.clone(),
            cached_at: now,
        };

        let json = serde_json::to_string_pretty(&cache).unwrap();
        let parsed: Cache = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed.repos.len(), 2);
        assert_eq!(parsed.cached_at, now);
        assert_eq!(parsed.repos[0].full_name, "user/repo1");
        assert_eq!(parsed.repos[1].stars, 50);
    }

    #[test]
    fn test_cache_age_formatting() {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let test_cases = vec![
            (now - 30, "s ago"),
            (now - 120, "m ago"),
            (now - 3660, "h ago"),
            (now - 86500, "d ago"),
        ];

        for (cached_at, expected_suffix) in test_cases {
            let cache = Cache {
                repos: vec![],
                cached_at,
            };

            let json = serde_json::to_string_pretty(&cache).unwrap();
            let parsed: Cache = serde_json::from_str(&json).unwrap();

            let age_secs = now - parsed.cached_at;
            let formatted = if age_secs < 60 {
                format!("{}s ago", age_secs)
            } else if age_secs < 3600 {
                format!("{}m ago", age_secs / 60)
            } else if age_secs < 86400 {
                format!("{}h ago", age_secs / 3600)
            } else {
                format!("{}d ago", age_secs / 86400)
            };

            assert!(formatted.ends_with(expected_suffix));
        }
    }

    #[test]
    fn test_github_repo_serialization() {
        let repo = GitHubRepo {
            full_name: "test/repo".to_string(),
            description: Some("Test description".to_string()),
            is_private: true,
            default_branch: "develop".to_string(),
            pushed_at: Utc::now(),
            language: Some("Rust".to_string()),
            stars: 123,
            url: "https://github.com/test/repo".to_string(),
        };

        let json = serde_json::to_string(&repo).unwrap();
        let parsed: GitHubRepo = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed.full_name, "test/repo");
        assert_eq!(parsed.description, Some("Test description".to_string()));
        assert_eq!(parsed.is_private, true);
        assert_eq!(parsed.stars, 123);
    }

    #[test]
    fn test_cache_structure() {
        let repos = vec![GitHubRepo {
            full_name: "owner/name".to_string(),
            description: None,
            is_private: false,
            default_branch: "main".to_string(),
            pushed_at: Utc::now(),
            language: Some("Go".to_string()),
            stars: 5,
            url: "https://github.com/owner/name".to_string(),
        }];

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let cache = Cache {
            repos: repos.clone(),
            cached_at: now,
        };

        assert_eq!(cache.repos.len(), 1);
        assert_eq!(cache.cached_at, now);
        assert_eq!(cache.repos[0].full_name, "owner/name");
    }

    #[test]
    fn test_empty_cache() {
        let cache = Cache {
            repos: vec![],
            cached_at: 0,
        };

        let json = serde_json::to_string(&cache).unwrap();
        let parsed: Cache = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed.repos.len(), 0);
        assert_eq!(parsed.cached_at, 0);
    }

    #[test]
    fn test_cache_ttl_boundary() {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        assert!(is_cache_valid(now));
        assert!(is_cache_valid(now - 1));
        assert!(is_cache_valid(now - (CACHE_TTL_SECS - 1)));
        assert!(!is_cache_valid(now - CACHE_TTL_SECS));
    }
}
