use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tracing::debug;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub projects_dir: String,
    pub github_user: String,
    pub editor: String,
    pub show_private: bool,
    #[serde(default)]
    pub checks: ChecksConfig,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChecksConfig {
    pub provider: String,
    pub checks: Vec<String>,
}

impl Default for ChecksConfig {
    fn default() -> Self {
        Self {
            provider: "claude".to_string(),
            checks: vec![
                "quality".to_string(),
                "logging".to_string(),
                "testing".to_string(),
                "security".to_string(),
            ],
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            projects_dir: "~/Projects".to_string(),
            github_user: "".to_string(),
            editor: "code".to_string(),
            show_private: true,
            checks: ChecksConfig::default(),
        }
    }
}

impl Config {
    pub fn load() -> Result<Self> {
        let config_path = Self::config_path()?;
        debug!(path = %config_path.display(), "Loading config");

        if !config_path.exists() {
            debug!("No config found, creating defaults");
            let config = Self::default();
            config.save()?;
            return Ok(config);
        }

        let contents = std::fs::read_to_string(&config_path)?;
        let config: Config = toml::from_str(&contents)?;
        debug!(projects_dir = %config.projects_dir, github_user = %config.github_user, "Config loaded");
        Ok(config)
    }

    pub fn save(&self) -> Result<()> {
        let config_path = Self::config_path()?;

        if let Some(parent) = config_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let contents = toml::to_string_pretty(self)?;
        std::fs::write(&config_path, contents)?;
        Ok(())
    }

    pub fn config_path() -> Result<PathBuf> {
        let config_dir = dirs::config_dir()
            .ok_or_else(|| anyhow::anyhow!("Could not determine config directory"))?;
        Ok(config_dir.join("proj").join("config.toml"))
    }

    pub fn projects_dir_expanded(&self) -> PathBuf {
        let path = shellexpand::tilde(&self.projects_dir).to_string();
        PathBuf::from(path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert_eq!(config.projects_dir, "~/Projects");
        assert_eq!(config.editor, "code");
        assert!(config.show_private);
    }

    #[test]
    fn test_config_roundtrip() {
        let config = Config::default();
        let toml_str = toml::to_string_pretty(&config).unwrap();
        let parsed: Config = toml::from_str(&toml_str).unwrap();
        assert_eq!(parsed.projects_dir, config.projects_dir);
        assert_eq!(parsed.editor, config.editor);
        assert_eq!(parsed.github_user, config.github_user);
    }

    #[test]
    fn test_projects_dir_expanded() {
        let config = Config {
            projects_dir: "~/Projects".to_string(),
            ..Config::default()
        };
        let expanded = config.projects_dir_expanded();
        assert!(!expanded.to_string_lossy().contains('~'));
        assert!(expanded.to_string_lossy().contains("Projects"));
    }

    #[test]
    fn test_checks_config_default() {
        let checks = ChecksConfig::default();
        assert_eq!(checks.provider, "claude");
        assert_eq!(checks.checks.len(), 4);
        assert!(checks.checks.contains(&"quality".to_string()));
        assert!(checks.checks.contains(&"security".to_string()));
        assert!(checks.checks.contains(&"logging".to_string()));
        assert!(checks.checks.contains(&"testing".to_string()));
    }

    #[test]
    fn test_config_with_custom_github_user() {
        let config = Config {
            github_user: "testuser".to_string(),
            ..Config::default()
        };
        let toml_str = toml::to_string_pretty(&config).unwrap();
        let parsed: Config = toml::from_str(&toml_str).unwrap();
        assert_eq!(parsed.github_user, "testuser");
    }

    #[test]
    fn test_checks_config_serialization() {
        let checks = ChecksConfig {
            provider: "custom".to_string(),
            checks: vec!["test1".to_string(), "test2".to_string()],
        };
        let config = Config {
            checks,
            ..Config::default()
        };
        let toml_str = toml::to_string_pretty(&config).unwrap();
        let parsed: Config = toml::from_str(&toml_str).unwrap();
        assert_eq!(parsed.checks.provider, "custom");
        assert_eq!(parsed.checks.checks.len(), 2);
    }
}
