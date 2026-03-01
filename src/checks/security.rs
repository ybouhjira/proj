use std::future::Future;
use std::path::Path;
use std::pin::Pin;
use tokio::process::Command;

use crate::project::CheckResult;

use super::detector::{ProjectInfo, ProjectLang};

pub async fn check_security(path: &Path, info: &ProjectInfo) -> CheckResult {
    let (vuln_count, audit_available) = match info.lang {
        ProjectLang::Rust => check_rust_security(path).await,
        ProjectLang::JavaScript | ProjectLang::TypeScript => check_js_security(path).await,
        ProjectLang::Python => check_python_security(path).await,
        ProjectLang::Go => check_go_security(path).await,
        ProjectLang::Unknown => (0, false),
    };

    let has_env_file = check_env_committed(path).await;
    let has_secrets = check_hardcoded_secrets(path).await;

    let no_vuln_score = if vuln_count == 0 { 0.4 } else { 0.0 };
    let no_secrets_score = if !has_secrets && !has_env_file {
        0.4
    } else {
        0.0
    };
    let audit_score = if audit_available { 0.2 } else { 0.0 };

    let score = no_vuln_score + no_secrets_score + audit_score;

    let mut suggestions = Vec::new();
    if vuln_count > 0 {
        suggestions.push(format!(
            "Fix {} security vulnerabilities in dependencies",
            vuln_count
        ));
    }
    if has_env_file {
        suggestions.push("Remove .env file from git - add to .gitignore".to_string());
    }
    if has_secrets {
        suggestions.push("Remove hardcoded secrets/API keys from code".to_string());
    }
    if !audit_available {
        let tool = match info.lang {
            ProjectLang::Rust => "cargo-audit",
            ProjectLang::JavaScript | ProjectLang::TypeScript => "npm audit",
            ProjectLang::Python => "pip-audit",
            _ => "security audit tool",
        };
        suggestions.push(format!("Install {} for vulnerability scanning", tool));
    }

    CheckResult {
        score,
        issues: vec![],
        suggestions,
    }
}

async fn check_rust_security(path: &Path) -> (usize, bool) {
    let output = Command::new("cargo")
        .args(["audit", "--json"])
        .current_dir(path)
        .output()
        .await;

    if let Ok(output) = output {
        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            if let Ok(json) = serde_json::from_str::<serde_json::Value>(&stdout) {
                let vuln_count = json
                    .get("vulnerabilities")
                    .and_then(|v| v.get("count"))
                    .and_then(|v| v.as_u64())
                    .unwrap_or(0);

                return (vuln_count as usize, true);
            }
        }
    }

    (0, false)
}

async fn check_js_security(path: &Path) -> (usize, bool) {
    let output = Command::new("npm")
        .args(["audit", "--json"])
        .current_dir(path)
        .output()
        .await;

    if let Ok(output) = output {
        let stdout = String::from_utf8_lossy(&output.stdout);
        if let Ok(json) = serde_json::from_str::<serde_json::Value>(&stdout) {
            let vuln_count = json
                .get("metadata")
                .and_then(|v| v.get("vulnerabilities"))
                .and_then(|v| v.get("total"))
                .and_then(|v| v.as_u64())
                .unwrap_or(0);

            return (vuln_count as usize, true);
        }
    }

    (0, false)
}

async fn check_python_security(path: &Path) -> (usize, bool) {
    let output = Command::new("pip-audit")
        .args(["--format=json"])
        .current_dir(path)
        .output()
        .await;

    if let Ok(output) = output {
        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            if let Ok(json) = serde_json::from_str::<serde_json::Value>(&stdout) {
                let vuln_count = json.as_array().map(|arr| arr.len()).unwrap_or(0);

                return (vuln_count, true);
            }
        }
    }

    (0, false)
}

async fn check_go_security(_path: &Path) -> (usize, bool) {
    // Go doesn't have a built-in audit tool like npm audit
    // We could use govulncheck but it's not standard
    (0, false)
}

async fn check_env_committed(path: &Path) -> bool {
    // Check if .env is tracked by git
    let output = Command::new("git")
        .args(["ls-files", ".env"])
        .current_dir(path)
        .output()
        .await;

    if let Ok(output) = output {
        !output.stdout.is_empty()
    } else {
        false
    }
}

async fn check_hardcoded_secrets(path: &Path) -> bool {
    let patterns = [
        "API_KEY=",
        "api_key:",
        "apiKey:",
        "password=",
        "PASSWORD=",
        "secret=",
        "SECRET=",
        "token=",
        "TOKEN=",
        "aws_access_key",
        "private_key",
    ];

    search_secrets_in_dir(path, &patterns).await
}

fn search_secrets_in_dir<'a>(
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

                    if search_secrets_in_dir(&path, patterns).await {
                        return true;
                    }
                } else if path.is_file() {
                    // Skip certain file types
                    if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                        if ext == "lock" || ext == "min" || ext == "map" {
                            continue;
                        }
                    }

                    if let Ok(content) = tokio::fs::read_to_string(&path).await {
                        // Only check first 10KB to avoid scanning large files
                        let preview = if content.len() > 10240 {
                            &content[..10240]
                        } else {
                            &content
                        };

                        for pattern in patterns {
                            if preview.contains(pattern) {
                                // Make sure it's not in a comment or documentation
                                if let Some(line) = preview.lines().find(|l| l.contains(pattern)) {
                                    let trimmed = line.trim();
                                    if !trimmed.starts_with("//")
                                        && !trimmed.starts_with('#')
                                        && !trimmed.starts_with('*')
                                    {
                                        return true;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        false
    })
}
