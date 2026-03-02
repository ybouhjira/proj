use anyhow::Result;
use std::future::Future;
use std::path::Path;
use std::pin::Pin;

#[derive(Debug, Clone, PartialEq)]
pub enum ProjectLang {
    Rust,
    JavaScript,
    TypeScript,
    Python,
    Go,
    Unknown,
}

impl std::fmt::Display for ProjectLang {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProjectLang::Rust => write!(f, "Rust"),
            ProjectLang::JavaScript => write!(f, "JavaScript"),
            ProjectLang::TypeScript => write!(f, "TypeScript"),
            ProjectLang::Python => write!(f, "Python"),
            ProjectLang::Go => write!(f, "Go"),
            ProjectLang::Unknown => write!(f, "Unknown"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ProjectInfo {
    pub lang: ProjectLang,
    pub has_tests: bool,
    pub has_linter: bool,
    pub has_formatter: bool,
    pub has_logging: bool,
    pub has_readme: bool,
    pub has_ci: bool,
    pub test_framework: Option<String>,
    pub linter: Option<String>,
}

pub async fn detect_project(path: &Path) -> Result<ProjectInfo> {
    let lang = detect_language(path).await;

    let has_readme = path.join("README.md").exists() || path.join("readme.md").exists();
    let has_ci = path.join(".github/workflows").exists()
        || path.join(".gitlab-ci.yml").exists()
        || path.join(".circleci").exists();

    match lang {
        ProjectLang::Rust => detect_rust_info(path, has_readme, has_ci).await,
        ProjectLang::JavaScript | ProjectLang::TypeScript => {
            detect_js_info(path, has_readme, has_ci).await
        }
        ProjectLang::Python => detect_python_info(path, has_readme, has_ci).await,
        ProjectLang::Go => detect_go_info(path, has_readme, has_ci).await,
        ProjectLang::Unknown => Ok(ProjectInfo {
            lang,
            has_tests: false,
            has_linter: false,
            has_formatter: false,
            has_logging: false,
            has_readme,
            has_ci,
            test_framework: None,
            linter: None,
        }),
    }
}

async fn detect_language(path: &Path) -> ProjectLang {
    if path.join("Cargo.toml").exists() {
        ProjectLang::Rust
    } else if path.join("package.json").exists() {
        // Check if TypeScript
        if path.join("tsconfig.json").exists() {
            ProjectLang::TypeScript
        } else {
            ProjectLang::JavaScript
        }
    } else if path.join("pyproject.toml").exists() || path.join("requirements.txt").exists() {
        ProjectLang::Python
    } else if path.join("go.mod").exists() {
        ProjectLang::Go
    } else {
        ProjectLang::Unknown
    }
}

async fn detect_rust_info(path: &Path, has_readme: bool, has_ci: bool) -> Result<ProjectInfo> {
    // Check for tests in src directory
    let has_tests = check_rust_tests(path).await;

    // Rust has clippy built-in
    let has_linter = true;
    let linter = Some("clippy".to_string());

    // Rust has rustfmt built-in
    let has_formatter = true;

    // Check for logging crates
    let has_logging = check_rust_logging(path).await;

    Ok(ProjectInfo {
        lang: ProjectLang::Rust,
        has_tests,
        has_linter,
        has_formatter,
        has_logging,
        has_readme,
        has_ci,
        test_framework: if has_tests {
            Some("built-in".to_string())
        } else {
            None
        },
        linter,
    })
}

async fn detect_js_info(path: &Path, has_readme: bool, has_ci: bool) -> Result<ProjectInfo> {
    let package_json_path = path.join("package.json");
    let lang = if path.join("tsconfig.json").exists() {
        ProjectLang::TypeScript
    } else {
        ProjectLang::JavaScript
    };

    let mut test_framework = None;
    let mut has_linter = false;
    let mut linter = None;

    // Read package.json
    if let Ok(content) = tokio::fs::read_to_string(&package_json_path).await {
        if let Ok(json) = serde_json::from_str::<serde_json::Value>(&content) {
            // Check for test frameworks
            if let Some(dev_deps) = json.get("devDependencies").and_then(|v| v.as_object()) {
                if dev_deps.contains_key("jest") {
                    test_framework = Some("jest".to_string());
                } else if dev_deps.contains_key("vitest") {
                    test_framework = Some("vitest".to_string());
                } else if dev_deps.contains_key("mocha") {
                    test_framework = Some("mocha".to_string());
                }

                // Check for linters
                if dev_deps.contains_key("eslint") {
                    has_linter = true;
                    linter = Some("eslint".to_string());
                }
            }
        }
    }

    let has_tests = test_framework.is_some() || check_js_tests(path).await;
    let has_formatter = path.join(".prettierrc").exists()
        || path.join("prettier.config.js").exists()
        || path.join(".prettierrc.json").exists();
    let has_logging = check_js_logging(path).await;

    Ok(ProjectInfo {
        lang,
        has_tests,
        has_linter,
        has_formatter,
        has_logging,
        has_readme,
        has_ci,
        test_framework,
        linter,
    })
}

async fn detect_python_info(path: &Path, has_readme: bool, has_ci: bool) -> Result<ProjectInfo> {
    let has_tests = check_python_tests(path).await;
    let test_framework = if has_tests {
        Some("pytest".to_string())
    } else {
        None
    };

    let has_linter = path.join(".flake8").exists()
        || path.join("ruff.toml").exists()
        || path.join("pyproject.toml").exists();
    let linter = if has_linter {
        Some("ruff/flake8".to_string())
    } else {
        None
    };

    let has_formatter = path.join(".black").exists() || has_linter;
    let has_logging = check_python_logging(path).await;

    Ok(ProjectInfo {
        lang: ProjectLang::Python,
        has_tests,
        has_linter,
        has_formatter,
        has_logging,
        has_readme,
        has_ci,
        test_framework,
        linter,
    })
}

async fn detect_go_info(path: &Path, has_readme: bool, has_ci: bool) -> Result<ProjectInfo> {
    let has_tests = check_go_tests(path).await;
    let test_framework = if has_tests {
        Some("built-in".to_string())
    } else {
        None
    };

    let has_linter = true; // Go has built-in gofmt and golint
    let has_formatter = true;
    let has_logging = check_go_logging(path).await;

    Ok(ProjectInfo {
        lang: ProjectLang::Go,
        has_tests,
        has_linter,
        has_formatter,
        has_logging,
        has_readme,
        has_ci,
        test_framework,
        linter: Some("golint".to_string()),
    })
}

async fn check_rust_tests(path: &Path) -> bool {
    // Check for #[cfg(test)] or #[test] in any .rs file
    let src_dir = path.join("src");
    if !src_dir.exists() {
        return false;
    }

    check_pattern_in_dir(&src_dir, &[r"#\[test\]", r"#\[cfg\(test\)\]"]).await
}

async fn check_rust_logging(path: &Path) -> bool {
    let cargo_toml = path.join("Cargo.toml");
    if !cargo_toml.exists() {
        return false;
    }

    if let Ok(content) = tokio::fs::read_to_string(&cargo_toml).await {
        return content.contains("tracing")
            || content.contains("log =")
            || content.contains("env_logger");
    }

    false
}

async fn check_js_tests(path: &Path) -> bool {
    // Check for test files
    check_pattern_in_dir(path, &[r"\.test\.", r"\.spec\.", r"__tests__"]).await
}

async fn check_js_logging(path: &Path) -> bool {
    check_pattern_in_dir(
        path,
        &[
            r#"require\(['"]winston['"\)]"#,
            r#"import.*from ['"]winston['"]"#,
            r#"from ['"]pino['"]"#,
        ],
    )
    .await
}

async fn check_python_tests(path: &Path) -> bool {
    // Look for test_*.py or *_test.py files
    check_pattern_in_dir(path, &[r"test_.*\.py", r".*_test\.py"]).await
}

async fn check_python_logging(path: &Path) -> bool {
    check_pattern_in_dir(path, &[r"import logging", r"from logging import"]).await
}

async fn check_go_tests(path: &Path) -> bool {
    check_pattern_in_dir(path, &[r"_test\.go"]).await
}

async fn check_go_logging(path: &Path) -> bool {
    check_pattern_in_dir(path, &[r#"import.*"log""#, r#"log\."#]).await
}

fn check_pattern_in_dir<'a>(
    dir: &'a Path,
    patterns: &'a [&'a str],
) -> Pin<Box<dyn Future<Output = bool> + 'a>> {
    Box::pin(async move {
        if !dir.exists() {
            return false;
        }

        // Simple recursive directory search
        if let Ok(mut entries) = tokio::fs::read_dir(dir).await {
            while let Ok(Some(entry)) = entries.next_entry().await {
                let path = entry.path();

                if path.is_dir() {
                    let dir_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
                    if dir_name == "node_modules" || dir_name == "target" || dir_name == ".git" {
                        continue;
                    }

                    if check_pattern_in_dir(&path, patterns).await {
                        return true;
                    }
                } else if path.is_file() {
                    if let Ok(content) = tokio::fs::read_to_string(&path).await {
                        for pattern in patterns {
                            if content.contains(&pattern.replace(r"\", "")) {
                                return true;
                            }
                        }
                    }
                }
            }
        }

        false
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use tokio::fs;

    #[tokio::test]
    async fn test_detect_language_by_cargo_toml() {
        let temp = tempdir().unwrap();
        let cargo_toml = temp.path().join("Cargo.toml");
        fs::write(&cargo_toml, "[package]\nname = \"test\"").await.unwrap();

        let lang = detect_language(temp.path()).await;
        assert_eq!(lang, ProjectLang::Rust);
    }

    #[tokio::test]
    async fn test_detect_language_by_package_json() {
        let temp = tempdir().unwrap();
        let package_json = temp.path().join("package.json");
        fs::write(&package_json, "{}").await.unwrap();

        let lang = detect_language(temp.path()).await;
        assert_eq!(lang, ProjectLang::JavaScript);
    }

    #[tokio::test]
    async fn test_detect_language_by_tsconfig() {
        let temp = tempdir().unwrap();
        let package_json = temp.path().join("package.json");
        let tsconfig = temp.path().join("tsconfig.json");
        fs::write(&package_json, "{}").await.unwrap();
        fs::write(&tsconfig, "{}").await.unwrap();

        let lang = detect_language(temp.path()).await;
        assert_eq!(lang, ProjectLang::TypeScript);
    }

    #[tokio::test]
    async fn test_detect_language_by_pyproject() {
        let temp = tempdir().unwrap();
        let pyproject = temp.path().join("pyproject.toml");
        fs::write(&pyproject, "[project]\nname = \"test\"").await.unwrap();

        let lang = detect_language(temp.path()).await;
        assert_eq!(lang, ProjectLang::Python);
    }

    #[tokio::test]
    async fn test_detect_language_by_requirements() {
        let temp = tempdir().unwrap();
        let requirements = temp.path().join("requirements.txt");
        fs::write(&requirements, "pytest\n").await.unwrap();

        let lang = detect_language(temp.path()).await;
        assert_eq!(lang, ProjectLang::Python);
    }

    #[tokio::test]
    async fn test_detect_language_by_go_mod() {
        let temp = tempdir().unwrap();
        let go_mod = temp.path().join("go.mod");
        fs::write(&go_mod, "module test\n").await.unwrap();

        let lang = detect_language(temp.path()).await;
        assert_eq!(lang, ProjectLang::Go);
    }

    #[tokio::test]
    async fn test_detect_language_unknown() {
        let temp = tempdir().unwrap();
        let lang = detect_language(temp.path()).await;
        assert_eq!(lang, ProjectLang::Unknown);
    }

    #[tokio::test]
    async fn test_project_info_rust_with_tests() {
        let temp = tempdir().unwrap();
        let cargo_toml = temp.path().join("Cargo.toml");
        fs::write(&cargo_toml, "[package]\nname = \"test\"").await.unwrap();

        let src_dir = temp.path().join("src");
        fs::create_dir(&src_dir).await.unwrap();
        let lib_rs = src_dir.join("lib.rs");
        fs::write(&lib_rs, "#[test]\nfn test_example() {}\n").await.unwrap();

        let info = detect_project(temp.path()).await.unwrap();
        assert_eq!(info.lang, ProjectLang::Rust);
        assert!(info.has_tests);
    }

    #[tokio::test]
    async fn test_project_info_has_readme() {
        let temp = tempdir().unwrap();
        let cargo_toml = temp.path().join("Cargo.toml");
        fs::write(&cargo_toml, "[package]\nname = \"test\"").await.unwrap();

        let readme = temp.path().join("README.md");
        fs::write(&readme, "# Test Project\n").await.unwrap();

        let info = detect_project(temp.path()).await.unwrap();
        assert!(info.has_readme);
    }

    #[tokio::test]
    async fn test_project_info_has_ci() {
        let temp = tempdir().unwrap();
        let cargo_toml = temp.path().join("Cargo.toml");
        fs::write(&cargo_toml, "[package]\nname = \"test\"").await.unwrap();

        let github_dir = temp.path().join(".github");
        fs::create_dir(&github_dir).await.unwrap();
        let workflows_dir = github_dir.join("workflows");
        fs::create_dir(&workflows_dir).await.unwrap();
        let ci_yml = workflows_dir.join("ci.yml");
        fs::write(&ci_yml, "name: CI\n").await.unwrap();

        let info = detect_project(temp.path()).await.unwrap();
        assert!(info.has_ci);
    }

    #[test]
    fn test_project_lang_display() {
        assert_eq!(ProjectLang::Rust.to_string(), "Rust");
        assert_eq!(ProjectLang::JavaScript.to_string(), "JavaScript");
        assert_eq!(ProjectLang::TypeScript.to_string(), "TypeScript");
        assert_eq!(ProjectLang::Python.to_string(), "Python");
        assert_eq!(ProjectLang::Go.to_string(), "Go");
        assert_eq!(ProjectLang::Unknown.to_string(), "Unknown");
    }
}
