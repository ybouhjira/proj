use std::future::Future;
use std::path::Path;
use std::pin::Pin;

use crate::project::CheckResult;

use super::detector::{ProjectInfo, ProjectLang};

pub async fn check_docs(path: &Path, info: &ProjectInfo) -> CheckResult {
    let has_readme = check_readme(path).await;
    let has_license = check_license(path).await;
    let has_contributing = check_contributing(path).await;
    let has_inline_docs = check_inline_docs(path, info).await;

    let readme_score = if has_readme { 0.3 } else { 0.0 };
    let license_score = if has_license { 0.2 } else { 0.0 };
    let inline_score = if has_inline_docs { 0.3 } else { 0.0 };
    let contributing_score = if has_contributing { 0.2 } else { 0.0 };

    let score = readme_score + license_score + inline_score + contributing_score;

    let mut suggestions = Vec::new();
    if !has_readme {
        suggestions.push("Add README.md with project description and usage".to_string());
    } else {
        // Check if README is substantial
        if let Ok(content) = tokio::fs::read_to_string(path.join("README.md")).await {
            if content.len() < 100 {
                suggestions.push("Expand README.md with more detailed documentation".to_string());
            }
        }
    }

    if !has_license {
        suggestions.push("Add LICENSE file".to_string());
    }

    if !has_inline_docs {
        let suggestion = match info.lang {
            ProjectLang::Rust => "Add rustdoc comments (///) to public items",
            ProjectLang::JavaScript | ProjectLang::TypeScript => "Add JSDoc comments to functions",
            ProjectLang::Python => "Add docstrings to functions and classes",
            ProjectLang::Go => "Add godoc comments to exported items",
            ProjectLang::Unknown => "Add inline documentation",
        };
        suggestions.push(suggestion.to_string());
    }

    if !has_contributing {
        suggestions.push("Add CONTRIBUTING.md for contributors".to_string());
    }

    CheckResult {
        score,
        issues: vec![],
        suggestions,
    }
}

async fn check_readme(path: &Path) -> bool {
    let readme_path = path.join("README.md");
    let readme_lower_path = path.join("readme.md");

    if readme_path.exists() {
        if let Ok(content) = tokio::fs::read_to_string(&readme_path).await {
            return content.len() > 100;
        }
    }

    if readme_lower_path.exists() {
        if let Ok(content) = tokio::fs::read_to_string(&readme_lower_path).await {
            return content.len() > 100;
        }
    }

    false
}

async fn check_license(path: &Path) -> bool {
    path.join("LICENSE").exists()
        || path.join("LICENSE.md").exists()
        || path.join("LICENCE").exists()
}

async fn check_contributing(path: &Path) -> bool {
    path.join("CONTRIBUTING.md").exists() || path.join("CONTRIBUTING").exists()
}

async fn check_inline_docs(path: &Path, info: &ProjectInfo) -> bool {
    match info.lang {
        ProjectLang::Rust => check_rustdoc(path).await,
        ProjectLang::JavaScript | ProjectLang::TypeScript => check_jsdoc(path).await,
        ProjectLang::Python => check_docstrings(path).await,
        ProjectLang::Go => check_godoc(path).await,
        ProjectLang::Unknown => false,
    }
}

async fn check_rustdoc(path: &Path) -> bool {
    let src_dir = path.join("src");
    search_pattern_in_dir(&src_dir, &["///", "//!"]).await
}

async fn check_jsdoc(path: &Path) -> bool {
    search_pattern_in_dir(path, &["/**", "@param", "@returns", "@type"]).await
}

async fn check_docstrings(path: &Path) -> bool {
    search_pattern_in_dir(path, &[r#"""""#, "'''", "Args:", "Returns:"]).await
}

async fn check_godoc(path: &Path) -> bool {
    // Go uses regular comments before declarations
    // This is a simplified check
    search_pattern_in_dir(path, &["// Package", "// func", "// type"]).await
}

fn search_pattern_in_dir<'a>(
    dir: &'a Path,
    patterns: &'a [&'a str],
) -> Pin<Box<dyn Future<Output = bool> + 'a>> {
    Box::pin(async move {
        if !dir.exists() {
            return false;
        }

        if let Ok(mut entries) = tokio::fs::read_dir(dir).await {
            while let Ok(Some(entry)) = entries.next_entry().await {
                let path = entry.path();

                if path.is_dir() {
                    let dir_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
                    if dir_name == "node_modules"
                        || dir_name == "target"
                        || dir_name == ".git"
                        || dir_name == "dist"
                        || dir_name == "build"
                    {
                        continue;
                    }

                    if search_pattern_in_dir(&path, patterns).await {
                        return true;
                    }
                } else if path.is_file() {
                    if let Ok(content) = tokio::fs::read_to_string(&path).await {
                        for pattern in patterns {
                            if content.contains(pattern) {
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
