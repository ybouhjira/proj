use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;

/// Stores project tags persistently
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub struct TagStore {
    /// Map of project name -> set of tags
    pub tags: HashMap<String, HashSet<String>>,
}

impl TagStore {
    /// Load tags from ~/.config/proj/tags.toml
    pub fn load() -> Result<Self> {
        todo!("Load from tags.toml")
    }

    /// Save tags to ~/.config/proj/tags.toml
    pub fn save(&self) -> Result<()> {
        todo!("Save to tags.toml")
    }

    /// Get path to tags file
    pub fn tags_path() -> Result<PathBuf> {
        let config_dir = dirs::config_dir()
            .ok_or_else(|| anyhow::anyhow!("Could not determine config directory"))?;
        Ok(config_dir.join("proj").join("tags.toml"))
    }

    /// Add a tag to a project
    pub fn add_tag(&mut self, project: &str, tag: &str) {
        self.tags
            .entry(project.to_string())
            .or_default()
            .insert(tag.to_string());
    }

    /// Remove a tag from a project
    pub fn remove_tag(&mut self, project: &str, tag: &str) -> bool {
        if let Some(tags) = self.tags.get_mut(project) {
            let removed = tags.remove(tag);
            if tags.is_empty() {
                self.tags.remove(project);
            }
            removed
        } else {
            false
        }
    }

    /// Get all tags for a project
    pub fn get_tags(&self, project: &str) -> Vec<String> {
        self.tags
            .get(project)
            .map(|tags| {
                let mut v: Vec<String> = tags.iter().cloned().collect();
                v.sort();
                v
            })
            .unwrap_or_default()
    }

    /// Get all projects with a specific tag
    pub fn projects_with_tag(&self, tag: &str) -> Vec<String> {
        let mut projects: Vec<String> = self
            .tags
            .iter()
            .filter(|(_, tags)| tags.contains(tag))
            .map(|(project, _)| project.clone())
            .collect();
        projects.sort();
        projects
    }

    /// Get all unique tags across all projects
    pub fn all_tags(&self) -> Vec<String> {
        let mut tags: Vec<String> = self
            .tags
            .values()
            .flat_map(|t| t.iter().cloned())
            .collect::<HashSet<_>>()
            .into_iter()
            .collect();
        tags.sort();
        tags
    }

    /// Clear all tags for a project
    pub fn clear_project_tags(&mut self, project: &str) {
        self.tags.remove(project);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_single_tag() {
        let mut store = TagStore::default();
        store.add_tag("myproject", "rust");

        let tags = store.get_tags("myproject");
        assert_eq!(tags, vec!["rust"]);
    }

    #[test]
    fn test_add_multiple_tags_to_same_project() {
        let mut store = TagStore::default();
        store.add_tag("myproject", "rust");
        store.add_tag("myproject", "cli");
        store.add_tag("myproject", "tool");

        let mut tags = store.get_tags("myproject");
        tags.sort();
        assert_eq!(tags, vec!["cli", "rust", "tool"]);
    }

    #[test]
    fn test_add_same_tag_twice_is_idempotent() {
        let mut store = TagStore::default();
        store.add_tag("myproject", "rust");
        store.add_tag("myproject", "rust");

        let tags = store.get_tags("myproject");
        assert_eq!(tags, vec!["rust"]);
        assert_eq!(tags.len(), 1);
    }

    #[test]
    fn test_remove_tag() {
        let mut store = TagStore::default();
        store.add_tag("myproject", "rust");
        store.add_tag("myproject", "cli");

        let removed = store.remove_tag("myproject", "rust");
        assert!(removed);

        let tags = store.get_tags("myproject");
        assert_eq!(tags, vec!["cli"]);
    }

    #[test]
    fn test_remove_nonexistent_tag_returns_false() {
        let mut store = TagStore::default();
        store.add_tag("myproject", "rust");

        let removed = store.remove_tag("myproject", "nonexistent");
        assert!(!removed);
    }

    #[test]
    fn test_remove_last_tag_removes_project_entry() {
        let mut store = TagStore::default();
        store.add_tag("myproject", "rust");

        store.remove_tag("myproject", "rust");

        assert!(!store.tags.contains_key("myproject"));
        assert_eq!(store.get_tags("myproject"), Vec::<String>::new());
    }

    #[test]
    fn test_get_tags_sorted() {
        let mut store = TagStore::default();
        store.add_tag("myproject", "zebra");
        store.add_tag("myproject", "alpha");
        store.add_tag("myproject", "beta");

        let tags = store.get_tags("myproject");
        assert_eq!(tags, vec!["alpha", "beta", "zebra"]);
    }

    #[test]
    fn test_get_tags_empty_project() {
        let store = TagStore::default();
        let tags = store.get_tags("nonexistent");
        assert_eq!(tags, Vec::<String>::new());
    }

    #[test]
    fn test_projects_with_tag() {
        let mut store = TagStore::default();
        store.add_tag("project1", "rust");
        store.add_tag("project2", "rust");
        store.add_tag("project3", "python");
        store.add_tag("project2", "cli");

        let projects = store.projects_with_tag("rust");
        assert_eq!(projects, vec!["project1", "project2"]);
    }

    #[test]
    fn test_projects_with_tag_none_found() {
        let mut store = TagStore::default();
        store.add_tag("project1", "rust");

        let projects = store.projects_with_tag("nonexistent");
        assert_eq!(projects, Vec::<String>::new());
    }

    #[test]
    fn test_all_tags_unique_and_sorted() {
        let mut store = TagStore::default();
        store.add_tag("project1", "rust");
        store.add_tag("project2", "rust");
        store.add_tag("project1", "cli");
        store.add_tag("project3", "python");
        store.add_tag("project2", "web");

        let all_tags = store.all_tags();
        assert_eq!(all_tags, vec!["cli", "python", "rust", "web"]);
    }

    #[test]
    fn test_clear_project_tags() {
        let mut store = TagStore::default();
        store.add_tag("myproject", "rust");
        store.add_tag("myproject", "cli");
        store.add_tag("other", "python");

        store.clear_project_tags("myproject");

        assert_eq!(store.get_tags("myproject"), Vec::<String>::new());
        assert_eq!(store.get_tags("other"), vec!["python"]);
    }

    #[test]
    fn test_clear_nonexistent_project() {
        let mut store = TagStore::default();
        store.add_tag("project1", "rust");

        store.clear_project_tags("nonexistent");

        // Should not panic, just no-op
        assert_eq!(store.get_tags("project1"), vec!["rust"]);
    }

    #[test]
    fn test_tag_store_serialization_roundtrip() {
        let mut store = TagStore::default();
        store.add_tag("project1", "rust");
        store.add_tag("project1", "cli");
        store.add_tag("project2", "python");

        // Serialize to TOML
        let toml_str = toml::to_string(&store).expect("Failed to serialize");

        // Deserialize back
        let deserialized: TagStore = toml::from_str(&toml_str).expect("Failed to deserialize");

        assert_eq!(store, deserialized);
    }

    #[test]
    fn test_tag_store_default_is_empty() {
        let store = TagStore::default();
        assert!(store.tags.is_empty());
        assert_eq!(store.all_tags(), Vec::<String>::new());
    }

    #[test]
    #[should_panic(expected = "not yet implemented")]
    fn test_load_not_implemented() {
        let _ = TagStore::load();
    }

    #[test]
    #[should_panic(expected = "not yet implemented")]
    fn test_save_not_implemented() {
        let store = TagStore::default();
        let _ = store.save();
    }

    #[test]
    fn test_tags_path_contains_config_dir() {
        let path = TagStore::tags_path().expect("Failed to get tags path");
        assert!(path.to_string_lossy().contains("proj"));
        assert!(path.to_string_lossy().ends_with("tags.toml"));
    }
}
