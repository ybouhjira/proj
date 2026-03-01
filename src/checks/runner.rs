use anyhow::Result;
use std::path::Path;

use crate::project::CheckResult;

use super::detector::{detect_project, ProjectLang};
use super::{docs, logging, quality, security, testing};

#[derive(Debug)]
pub struct CheckReport {
    pub project_name: String,
    pub overall_score: f32,
    pub checks: Vec<(String, CheckResult)>,
    pub lang: ProjectLang,
}

pub async fn run_checks(
    path: &Path,
    specific_check: Option<&str>,
    run_all: bool,
    _use_ai: bool,
) -> Result<CheckReport> {
    let project_name = path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("project")
        .to_string();

    let info = detect_project(path).await?;

    let mut checks = Vec::new();

    // Determine which checks to run
    let should_run_quality =
        run_all || specific_check.is_none() || specific_check == Some("quality");
    let should_run_testing =
        run_all || specific_check.is_none() || specific_check == Some("testing");
    let should_run_logging =
        run_all || specific_check.is_none() || specific_check == Some("logging");
    let should_run_security =
        run_all || specific_check.is_none() || specific_check == Some("security");
    let should_run_docs = run_all || specific_check.is_none() || specific_check == Some("docs");

    // Run checks
    if should_run_quality {
        let result = quality::check_quality(path, &info).await;
        checks.push(("quality".to_string(), result));
    }

    if should_run_testing {
        let result = testing::check_testing(path, &info).await;
        checks.push(("testing".to_string(), result));
    }

    if should_run_logging {
        let result = logging::check_logging(path, &info).await;
        checks.push(("logging".to_string(), result));
    }

    if should_run_security {
        let result = security::check_security(path, &info).await;
        checks.push(("security".to_string(), result));
    }

    if should_run_docs {
        let result = docs::check_docs(path, &info).await;
        checks.push(("docs".to_string(), result));
    }

    // Calculate overall score
    let overall_score = if !checks.is_empty() {
        checks.iter().map(|(_, r)| r.score).sum::<f32>() / checks.len() as f32
    } else {
        0.0
    };

    Ok(CheckReport {
        project_name,
        overall_score,
        checks,
        lang: info.lang,
    })
}
