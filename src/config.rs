use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

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

        if !config_path.exists() {
            let config = Self::default();
            config.save()?;
            return Ok(config);
        }

        let contents = std::fs::read_to_string(&config_path)?;
        let config: Config = toml::from_str(&contents)?;
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
