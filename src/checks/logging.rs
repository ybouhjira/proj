use std::future::Future;
use std::path::Path;
use std::pin::Pin;

use crate::project::CheckResult;

use super::detector::{ProjectInfo, ProjectLang};

pub async fn check_logging(path: &Path, info: &ProjectInfo) -> CheckResult {
    let (has_structured, has_raw_prints) = match info.lang {
        ProjectLang::Rust => check_rust_logging(path).await,
        ProjectLang::JavaScript | ProjectLang::TypeScript => check_js_logging(path).await,
        ProjectLang::Python => check_python_logging(path).await,
        ProjectLang::Go => check_go_logging(path).await,
        ProjectLang::Unknown => (false, false),
    };

    let structured_score = if has_structured { 0.4 } else { 0.0 };
    let no_raw_score = if !has_raw_prints { 0.3 } else { 0.0 };
    let consistent_score = if has_structured && !has_raw_prints {
        0.3
    } else {
        0.0
    };

    let score = structured_score + no_raw_score + consistent_score;

    let mut suggestions = Vec::new();
    if !has_structured {
        let suggestion = match info.lang {
            ProjectLang::Rust => "Add structured logging with tracing or log crate",
            ProjectLang::JavaScript | ProjectLang::TypeScript => {
                "Add structured logging with winston or pino"
            }
            ProjectLang::Python => "Add structured logging with Python's logging module",
            ProjectLang::Go => "Use structured logging with log/slog",
            ProjectLang::Unknown => "Add structured logging framework",
        };
        suggestions.push(suggestion.to_string());
    }

    if has_raw_prints {
        let suggestion = match info.lang {
            ProjectLang::Rust => "Replace println! with proper logging (tracing/log)",
            ProjectLang::JavaScript | ProjectLang::TypeScript => {
                "Replace console.log with structured logger"
            }
            ProjectLang::Python => "Replace print() with logging.info/debug",
            ProjectLang::Go => "Replace fmt.Println with log.Printf",
            ProjectLang::Unknown => "Replace raw print statements with logging",
        };
        suggestions.push(suggestion.to_string());
    }

    CheckResult {
        score,
        issues: vec![],
        suggestions,
    }
}

async fn check_rust_logging(path: &Path) -> (bool, bool) {
    let src_dir = path.join("src");
    if !src_dir.exists() {
        return (false, false);
    }

    let has_structured = search_in_dir(&src_dir, &["tracing::", "log::", "env_logger"]).await;
    let has_raw_prints = search_in_dir(&src_dir, &["println!", "eprintln!", "dbg!"]).await;

    (has_structured, has_raw_prints)
}

async fn check_js_logging(path: &Path) -> (bool, bool) {
    let has_structured = search_in_dir(
        path,
        &["winston", "pino", "bunyan", "logger.info", "logger.error"],
    )
    .await;
    let has_raw_prints =
        search_in_dir(path, &["console.log", "console.error", "console.warn"]).await;

    (has_structured, has_raw_prints)
}

async fn check_python_logging(path: &Path) -> (bool, bool) {
    let has_structured = search_in_dir(
        path,
        &["import logging", "logging.info", "logging.error", "logger."],
    )
    .await;
    let has_raw_prints = search_in_dir(path, &["print("]).await;

    (has_structured, has_raw_prints)
}

async fn check_go_logging(path: &Path) -> (bool, bool) {
    let has_structured = search_in_dir(path, &["log.Printf", "log.Info", "slog."]).await;
    let has_raw_prints = search_in_dir(path, &["fmt.Println", "fmt.Printf"]).await;

    (has_structured, has_raw_prints)
}

fn search_in_dir<'a>(
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

                    if search_in_dir(&path, patterns).await {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::checks::detector::{ProjectInfo, ProjectLang};
    use tempfile::tempdir;
    use tokio::fs;

    #[tokio::test]
    async fn test_check_logging_rust_with_tracing() {
        let temp = tempdir().unwrap();
        let src_dir = temp.path().join("src");
        fs::create_dir(&src_dir).await.unwrap();
        let main_rs = src_dir.join("main.rs");
        fs::write(&main_rs, "use tracing::info;\nfn main() { info!(\"test\"); }").await.unwrap();

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

        let result = check_logging(temp.path(), &info).await;
        assert!(result.score > 0.5);
        assert!(result.suggestions.is_empty());
    }

    #[tokio::test]
    async fn test_check_logging_rust_with_println() {
        let temp = tempdir().unwrap();
        let src_dir = temp.path().join("src");
        fs::create_dir(&src_dir).await.unwrap();
        let main_rs = src_dir.join("main.rs");
        fs::write(&main_rs, "fn main() { println!(\"test\"); }").await.unwrap();

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

        let result = check_logging(temp.path(), &info).await;
        assert!(result.score < 0.5);
        assert!(!result.suggestions.is_empty());
        assert!(result.suggestions.iter().any(|s| s.contains("println!")));
    }

    #[tokio::test]
    async fn test_check_logging_rust_both() {
        let temp = tempdir().unwrap();
        let src_dir = temp.path().join("src");
        fs::create_dir(&src_dir).await.unwrap();
        let main_rs = src_dir.join("main.rs");
        fs::write(&main_rs, "use tracing::info;\nfn main() { info!(\"test\"); println!(\"debug\"); }").await.unwrap();

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

        let result = check_logging(temp.path(), &info).await;
        assert!(result.score > 0.0 && result.score < 1.0);
        assert!(result.suggestions.iter().any(|s| s.contains("println!")));
    }

    #[tokio::test]
    async fn test_check_logging_rust_neither() {
        let temp = tempdir().unwrap();
        let src_dir = temp.path().join("src");
        fs::create_dir(&src_dir).await.unwrap();
        let main_rs = src_dir.join("main.rs");
        fs::write(&main_rs, "fn main() {}").await.unwrap();

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

        let result = check_logging(temp.path(), &info).await;
        assert_eq!(result.score, 0.3);
    }

    #[tokio::test]
    async fn test_check_logging_js_with_winston() {
        let temp = tempdir().unwrap();
        let index_js = temp.path().join("index.js");
        fs::write(&index_js, "console.log('test');").await.unwrap();

        let info = ProjectInfo {
            lang: ProjectLang::JavaScript,
            has_tests: false,
            has_linter: false,
            has_formatter: false,
            has_logging: false,
            has_readme: false,
            has_ci: false,
            test_framework: None,
            linter: None,
        };

        let result = check_logging(temp.path(), &info).await;
        assert!(!result.suggestions.is_empty());
        assert!(result.suggestions.iter().any(|s| s.contains("structured logger")));
    }

    #[tokio::test]
    async fn test_check_logging_python_with_logging_module() {
        let temp = tempdir().unwrap();
        let main_py = temp.path().join("main.py");
        fs::write(&main_py, "import logging\nlogging.info('test')").await.unwrap();

        let info = ProjectInfo {
            lang: ProjectLang::Python,
            has_tests: false,
            has_linter: false,
            has_formatter: false,
            has_logging: false,
            has_readme: false,
            has_ci: false,
            test_framework: None,
            linter: None,
        };

        let result = check_logging(temp.path(), &info).await;
        assert!(result.score > 0.5);
    }

    #[tokio::test]
    async fn test_check_logging_unknown_language() {
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

        let result = check_logging(temp.path(), &info).await;
        assert_eq!(result.score, 0.3);
    }
}
