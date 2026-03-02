use std::future::Future;
use std::path::Path;
use std::pin::Pin;
use tokio::process::Command;

use crate::project::CheckResult;

use super::detector::{ProjectInfo, ProjectLang};

pub async fn check_testing(path: &Path, info: &ProjectInfo) -> CheckResult {
    if !info.has_tests {
        return CheckResult {
            score: 0.0,
            issues: vec![],
            suggestions: vec!["No test framework detected - add tests to your project".to_string()],
        };
    }

    let (test_count, source_count) = match info.lang {
        ProjectLang::Rust => count_rust_tests(path).await,
        ProjectLang::JavaScript | ProjectLang::TypeScript => count_js_tests(path, info).await,
        ProjectLang::Python => count_python_tests(path).await,
        ProjectLang::Go => count_go_tests(path).await,
        ProjectLang::Unknown => (0, 0),
    };

    let test_ratio = if source_count > 0 {
        test_count as f32 / source_count as f32
    } else {
        0.0
    };

    let has_tests_score = if info.has_tests { 0.4 } else { 0.0 };
    let ratio_score = if test_ratio > 0.3 {
        0.3
    } else {
        test_ratio * 0.3 / 0.3
    };
    let framework_score = if info.test_framework.is_some() {
        0.3
    } else {
        0.0
    };

    let score = has_tests_score + ratio_score + framework_score;

    let mut suggestions = Vec::new();
    if test_ratio < 0.3 {
        suggestions.push(format!(
            "Improve test coverage - {} test files / {} source files ({}%)",
            test_count,
            source_count,
            (test_ratio * 100.0) as u32
        ));
    }
    if info.test_framework.is_none() {
        suggestions.push("Configure a test framework".to_string());
    }

    CheckResult {
        score,
        issues: vec![],
        suggestions,
    }
}

async fn count_rust_tests(path: &Path) -> (usize, usize) {
    // Count test modules and source files
    let test_output = Command::new("cargo")
        .args(["test", "--", "--list"])
        .current_dir(path)
        .output()
        .await;

    let test_count = if let Ok(output) = test_output {
        let stdout = String::from_utf8_lossy(&output.stdout);
        stdout.lines().filter(|line| line.contains("test")).count()
    } else {
        0
    };

    let source_count = count_files_recursive(path, &["rs"]).await;

    (test_count.max(1), source_count.max(1))
}

async fn count_js_tests(path: &Path, info: &ProjectInfo) -> (usize, usize) {
    if let Some(ref framework) = info.test_framework {
        let test_count = match framework.as_str() {
            "jest" => {
                let output = Command::new("npx")
                    .args(["jest", "--listTests"])
                    .current_dir(path)
                    .output()
                    .await;

                if let Ok(output) = output {
                    let stdout = String::from_utf8_lossy(&output.stdout);
                    stdout.lines().count()
                } else {
                    0
                }
            }
            _ => count_files_recursive(path, &["test.ts", "test.js", "spec.ts", "spec.js"]).await,
        };

        let source_count = count_files_recursive(path, &["ts", "tsx", "js", "jsx"]).await;

        (test_count.max(1), source_count.max(1))
    } else {
        (0, 1)
    }
}

async fn count_python_tests(path: &Path) -> (usize, usize) {
    let test_count = count_files_with_pattern(path, "test_").await
        + count_files_with_pattern(path, "_test.py").await;
    let source_count = count_files_recursive(path, &["py"]).await;

    (test_count.max(1), source_count.max(1))
}

async fn count_go_tests(path: &Path) -> (usize, usize) {
    let test_count = count_files_with_pattern(path, "_test.go").await;
    let source_count = count_files_recursive(path, &["go"]).await;

    (test_count.max(1), source_count.max(1))
}

fn count_files_recursive<'a>(
    dir: &'a Path,
    extensions: &'a [&'a str],
) -> Pin<Box<dyn Future<Output = usize> + 'a>> {
    Box::pin(async move {
        let mut count = 0;

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

                    count += count_files_recursive(&path, extensions).await;
                } else if path.is_file() {
                    if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                        if extensions.contains(&ext) {
                            count += 1;
                        }
                    }

                    // Also check for patterns like .test.js
                    if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
                        for ext in extensions {
                            if file_name.ends_with(ext) {
                                count += 1;
                                break;
                            }
                        }
                    }
                }
            }
        }

        count
    })
}

fn count_files_with_pattern<'a>(
    dir: &'a Path,
    pattern: &'a str,
) -> Pin<Box<dyn Future<Output = usize> + 'a>> {
    Box::pin(async move {
        let mut count = 0;

        if let Ok(mut entries) = tokio::fs::read_dir(dir).await {
            while let Ok(Some(entry)) = entries.next_entry().await {
                let path = entry.path();

                if path.is_dir() {
                    let dir_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
                    if dir_name == "node_modules" || dir_name == "target" || dir_name == ".git" {
                        continue;
                    }

                    count += count_files_with_pattern(&path, pattern).await;
                } else if path.is_file() {
                    if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
                        if file_name.contains(pattern) {
                            count += 1;
                        }
                    }
                }
            }
        }

        count
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::checks::detector::{ProjectInfo, ProjectLang};
    use tempfile::tempdir;

    #[tokio::test]
    async fn test_check_testing_no_tests() {
        let temp = tempdir().unwrap();

        let info = ProjectInfo {
            lang: ProjectLang::Rust,
            has_tests: false,
            has_linter: false,
            has_formatter: false,
            has_logging: false,
            has_readme: false,
            has_ci: false,
            test_framework: None,
            linter: None,
        };

        let result = check_testing(temp.path(), &info).await;
        assert_eq!(result.score, 0.0);
        assert!(!result.suggestions.is_empty());
        assert!(result.suggestions.iter().any(|s| s.contains("No test framework")));
    }

    #[tokio::test]
    async fn test_check_testing_with_framework() {
        let temp = tempdir().unwrap();

        let info = ProjectInfo {
            lang: ProjectLang::JavaScript,
            has_tests: true,
            has_linter: false,
            has_formatter: false,
            has_logging: false,
            has_readme: false,
            has_ci: false,
            test_framework: Some("jest".to_string()),
            linter: None,
        };

        let result = check_testing(temp.path(), &info).await;
        assert!(result.score >= 0.7);
    }

    #[tokio::test]
    async fn test_check_testing_unknown_lang() {
        let temp = tempdir().unwrap();

        let info = ProjectInfo {
            lang: ProjectLang::Unknown,
            has_tests: false,
            has_linter: false,
            has_formatter: false,
            has_logging: false,
            has_readme: false,
            has_ci: false,
            test_framework: None,
            linter: None,
        };

        let result = check_testing(temp.path(), &info).await;
        assert_eq!(result.score, 0.0);
    }
}
