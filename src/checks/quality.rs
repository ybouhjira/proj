use std::path::Path;
use tokio::process::Command;

use crate::project::{CheckResult, Issue, Severity};

use super::detector::{ProjectInfo, ProjectLang};

pub async fn check_quality(path: &Path, info: &ProjectInfo) -> CheckResult {
    match info.lang {
        ProjectLang::Rust => check_rust_quality(path).await,
        ProjectLang::JavaScript | ProjectLang::TypeScript => check_js_quality(path, info).await,
        ProjectLang::Python => check_python_quality(path).await,
        ProjectLang::Go => check_go_quality(path).await,
        ProjectLang::Unknown => CheckResult {
            score: 0.5,
            issues: vec![],
            suggestions: vec!["Unknown project language - cannot run quality checks".to_string()],
        },
    }
}

async fn check_rust_quality(path: &Path) -> CheckResult {
    let output = Command::new("cargo")
        .args(["clippy", "--message-format=json", "--", "-D", "warnings"])
        .current_dir(path)
        .output()
        .await;

    if let Ok(output) = output {
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            let stdout = String::from_utf8_lossy(&output.stdout);

            let mut issues = Vec::new();
            let mut warning_count = 0;
            let mut error_count = 0;

            // Parse JSON output
            for line in stdout.lines() {
                if let Ok(json) = serde_json::from_str::<serde_json::Value>(line) {
                    if let Some(message) = json.get("message") {
                        if let Some(msg_text) = message.get("message").and_then(|v| v.as_str()) {
                            let level = message
                                .get("level")
                                .and_then(|v| v.as_str())
                                .unwrap_or("warning");

                            let severity = if level == "error" {
                                error_count += 1;
                                Severity::Error
                            } else {
                                warning_count += 1;
                                Severity::Warning
                            };

                            issues.push(Issue {
                                severity,
                                message: msg_text.to_string(),
                                file: None,
                                line: None,
                            });
                        }
                    }
                }
            }

            // Also check stderr for compilation errors
            if !stderr.is_empty() && stderr.contains("error") {
                error_count += 1;
            }

            let total_issues = warning_count + error_count;
            let score = 1.0 - (total_issues as f32 / 50.0).min(1.0);

            let mut suggestions = Vec::new();
            if warning_count > 0 {
                suggestions.push(format!("Fix {} clippy warnings", warning_count));
            }
            if error_count > 0 {
                suggestions.push(format!("Fix {} errors", error_count));
            }

            CheckResult {
                score,
                issues,
                suggestions,
            }
        } else {
            CheckResult {
                score: 1.0,
                issues: vec![],
                suggestions: vec![],
            }
        }
    } else {
        CheckResult {
            score: 0.5,
            issues: vec![],
            suggestions: vec![
                "Could not run cargo clippy - ensure Rust toolchain is installed".to_string(),
            ],
        }
    }
}

async fn check_js_quality(path: &Path, info: &ProjectInfo) -> CheckResult {
    if !info.has_linter {
        return CheckResult {
            score: 0.5,
            issues: vec![],
            suggestions: vec!["Install ESLint for code quality checks".to_string()],
        };
    }

    let output = Command::new("npx")
        .args(["eslint", ".", "--format=json"])
        .current_dir(path)
        .output()
        .await;

    if let Ok(output) = output {
        let stdout = String::from_utf8_lossy(&output.stdout);

        if let Ok(json) = serde_json::from_str::<serde_json::Value>(&stdout) {
            let mut issues = Vec::new();
            let mut total_issues = 0;

            if let Some(results) = json.as_array() {
                for result in results {
                    if let Some(messages) = result.get("messages").and_then(|v| v.as_array()) {
                        for message in messages {
                            let severity_level = message
                                .get("severity")
                                .and_then(|v| v.as_u64())
                                .unwrap_or(1);

                            let severity = if severity_level == 2 {
                                Severity::Error
                            } else {
                                Severity::Warning
                            };

                            let msg = message
                                .get("message")
                                .and_then(|v| v.as_str())
                                .unwrap_or("Unknown issue");

                            issues.push(Issue {
                                severity,
                                message: msg.to_string(),
                                file: None,
                                line: None,
                            });

                            total_issues += 1;
                        }
                    }
                }
            }

            let score = 1.0 - (total_issues as f32 / 50.0).min(1.0);

            let mut suggestions = Vec::new();
            if total_issues > 0 {
                suggestions.push(format!("Fix {} ESLint issues", total_issues));
            }

            CheckResult {
                score,
                issues,
                suggestions,
            }
        } else {
            CheckResult {
                score: 1.0,
                issues: vec![],
                suggestions: vec![],
            }
        }
    } else {
        CheckResult {
            score: 0.5,
            issues: vec![],
            suggestions: vec!["Could not run ESLint - ensure it's installed".to_string()],
        }
    }
}

async fn check_python_quality(path: &Path) -> CheckResult {
    // Try ruff first, then flake8
    let output = Command::new("ruff")
        .args(["check", ".", "--output-format=json"])
        .current_dir(path)
        .output()
        .await;

    if let Ok(output) = output {
        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);

            if let Ok(json) = serde_json::from_str::<serde_json::Value>(&stdout) {
                let issues_count = json.as_array().map(|arr| arr.len()).unwrap_or(0);
                let score = 1.0 - (issues_count as f32 / 50.0).min(1.0);

                let suggestions = if issues_count > 0 {
                    vec![format!("Fix {} ruff issues", issues_count)]
                } else {
                    vec![]
                };

                return CheckResult {
                    score,
                    issues: vec![],
                    suggestions,
                };
            }
        }
    }

    // Fallback to simple score
    CheckResult {
        score: 0.7,
        issues: vec![],
        suggestions: vec!["Install ruff for code quality checks".to_string()],
    }
}

async fn check_go_quality(path: &Path) -> CheckResult {
    let output = Command::new("go")
        .args(["vet", "./..."])
        .current_dir(path)
        .output()
        .await;

    if let Ok(output) = output {
        if output.status.success() {
            CheckResult {
                score: 1.0,
                issues: vec![],
                suggestions: vec![],
            }
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            let issue_count = stderr.lines().count();
            let score = 1.0 - (issue_count as f32 / 50.0).min(1.0);

            CheckResult {
                score,
                issues: vec![],
                suggestions: vec![format!("Fix {} go vet issues", issue_count)],
            }
        }
    } else {
        CheckResult {
            score: 0.7,
            issues: vec![],
            suggestions: vec!["Could not run go vet - ensure Go is installed".to_string()],
        }
    }
}
