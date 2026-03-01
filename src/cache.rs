use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::project::GitHubRepo;

const CACHE_TTL_SECS: u64 = 300; // 5 minutes

#[derive(Serialize, Deserialize)]
struct Cache {
    repos: Vec<GitHubRepo>,
    cached_at: u64, // unix timestamp
}

/// Get cached repos or None if cache is stale/missing
pub fn get_cached_repos() -> Option<Vec<GitHubRepo>> {
    let path = cache_path();

    if !path.exists() {
        return None;
    }

    let contents = fs::read_to_string(&path).ok()?;
    let cache: Cache = serde_json::from_str(&contents).ok()?;

    if !is_cache_valid(cache.cached_at) {
        return None;
    }

    Some(cache.repos)
}

/// Save repos to cache
pub fn save_cache(repos: &[GitHubRepo]) -> Result<()> {
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
