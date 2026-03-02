use anyhow::Result;
use std::collections::HashMap;

/// Aggregate statistics across all projects
#[derive(Debug, Default, Clone, PartialEq)]
pub struct PortfolioStats {
    /// Lines of code by language
    pub loc_by_language: HashMap<String, usize>,
    /// Total lines of code
    pub total_loc: usize,
    /// Number of projects analyzed
    pub project_count: usize,
    /// Total GitHub stars across all repos
    pub total_stars: u32,
    /// Total GitHub forks across all repos
    pub total_forks: u32,
    /// Most active projects by recent commit count (name, count)
    pub most_active: Vec<(String, usize)>,
    /// Languages used across all projects
    pub languages: Vec<(String, usize)>,
    /// Commit streak: consecutive days with commits
    pub commit_streak: u32,
    /// Total commits across all projects
    pub total_commits: usize,
}

/// Count lines of code in a single file, excluding blank lines and comments
pub fn count_loc(content: &str, language: &str) -> usize {
    todo!("Count LOC excluding blanks and comments")
}

/// Detect language from file extension
pub fn detect_language_from_ext(ext: &str) -> Option<String> {
    match ext {
        "rs" => Some("Rust".to_string()),
        "js" | "jsx" => Some("JavaScript".to_string()),
        "ts" | "tsx" => Some("TypeScript".to_string()),
        "py" => Some("Python".to_string()),
        "go" => Some("Go".to_string()),
        "java" => Some("Java".to_string()),
        "rb" => Some("Ruby".to_string()),
        "c" | "h" => Some("C".to_string()),
        "cpp" | "cc" | "cxx" | "hpp" => Some("C++".to_string()),
        "sh" | "bash" => Some("Shell".to_string()),
        "toml" => Some("TOML".to_string()),
        "yaml" | "yml" => Some("YAML".to_string()),
        "json" => Some("JSON".to_string()),
        "md" => Some("Markdown".to_string()),
        "html" => Some("HTML".to_string()),
        "css" | "scss" => Some("CSS".to_string()),
        _ => None,
    }
}

/// Calculate commit streak from a list of commit dates (sorted descending)
pub fn calculate_streak(commit_dates: &[chrono::NaiveDate]) -> u32 {
    todo!("Calculate consecutive day streak ending today or yesterday")
}

/// Merge language stats from multiple projects
pub fn merge_language_stats(stats: &[HashMap<String, usize>]) -> Vec<(String, usize)> {
    todo!("Merge and sort by count descending")
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    #[test]
    fn test_detect_language_rust() {
        assert_eq!(detect_language_from_ext("rs"), Some("Rust".to_string()));
    }

    #[test]
    fn test_detect_language_javascript() {
        assert_eq!(
            detect_language_from_ext("js"),
            Some("JavaScript".to_string())
        );
        assert_eq!(
            detect_language_from_ext("jsx"),
            Some("JavaScript".to_string())
        );
    }

    #[test]
    fn test_detect_language_typescript() {
        assert_eq!(
            detect_language_from_ext("ts"),
            Some("TypeScript".to_string())
        );
        assert_eq!(
            detect_language_from_ext("tsx"),
            Some("TypeScript".to_string())
        );
    }

    #[test]
    fn test_detect_language_python() {
        assert_eq!(detect_language_from_ext("py"), Some("Python".to_string()));
    }

    #[test]
    fn test_detect_language_go() {
        assert_eq!(detect_language_from_ext("go"), Some("Go".to_string()));
    }

    #[test]
    fn test_detect_language_unknown() {
        assert_eq!(detect_language_from_ext("unknown"), None);
        assert_eq!(detect_language_from_ext("xyz"), None);
        assert_eq!(detect_language_from_ext(""), None);
    }

    #[test]
    fn test_detect_language_cpp_variants() {
        assert_eq!(detect_language_from_ext("cpp"), Some("C++".to_string()));
        assert_eq!(detect_language_from_ext("cc"), Some("C++".to_string()));
        assert_eq!(detect_language_from_ext("cxx"), Some("C++".to_string()));
        assert_eq!(detect_language_from_ext("hpp"), Some("C++".to_string()));
    }

    #[test]
    fn test_detect_language_shell() {
        assert_eq!(detect_language_from_ext("sh"), Some("Shell".to_string()));
        assert_eq!(detect_language_from_ext("bash"), Some("Shell".to_string()));
    }

    #[test]
    fn test_detect_language_web() {
        assert_eq!(detect_language_from_ext("html"), Some("HTML".to_string()));
        assert_eq!(detect_language_from_ext("css"), Some("CSS".to_string()));
        assert_eq!(detect_language_from_ext("scss"), Some("CSS".to_string()));
    }

    #[test]
    #[should_panic(expected = "not yet implemented")]
    fn test_count_loc_rust() {
        let code = r#"
// This is a comment
fn main() {
    println!("Hello, world!");
    // Another comment

    let x = 42;
}
"#;
        let loc = count_loc(code, "Rust");
        // Should count 4 lines: fn main, println, blank line inside fn, let x
        assert_eq!(loc, 4);
    }

    #[test]
    #[should_panic(expected = "not yet implemented")]
    fn test_count_loc_python() {
        let code = r#"
# This is a comment
def hello():
    print("Hello")
    # Another comment
    x = 42
    return x
"#;
        let loc = count_loc(code, "Python");
        // Should count 4 lines: def hello, print, x = 42, return x
        assert_eq!(loc, 4);
    }

    #[test]
    #[should_panic(expected = "not yet implemented")]
    fn test_count_loc_javascript() {
        let code = r#"
// Single line comment
function hello() {
    console.log("Hello");
    /* Multi-line
       comment */
    const x = 42;
}
"#;
        let loc = count_loc(code, "JavaScript");
        // Should count 3 lines: function hello, console.log, const x
        assert_eq!(loc, 3);
    }

    #[test]
    #[should_panic(expected = "not yet implemented")]
    fn test_count_loc_empty_string() {
        let loc = count_loc("", "Rust");
        assert_eq!(loc, 0);
    }

    #[test]
    #[should_panic(expected = "not yet implemented")]
    fn test_count_loc_only_comments() {
        let code = r#"
// Comment 1
// Comment 2
// Comment 3
"#;
        let loc = count_loc(code, "Rust");
        assert_eq!(loc, 0);
    }

    #[test]
    #[should_panic(expected = "not yet implemented")]
    fn test_count_loc_only_blank_lines() {
        let code = "\n\n\n\n";
        let loc = count_loc(code, "Rust");
        assert_eq!(loc, 0);
    }

    #[test]
    #[should_panic(expected = "not yet implemented")]
    fn test_calculate_streak_consecutive_days() {
        let today = chrono::Local::now().date_naive();
        let dates = vec![
            today,
            today - chrono::Duration::days(1),
            today - chrono::Duration::days(2),
            today - chrono::Duration::days(3),
        ];
        let streak = calculate_streak(&dates);
        assert_eq!(streak, 4);
    }

    #[test]
    #[should_panic(expected = "not yet implemented")]
    fn test_calculate_streak_with_gap() {
        let today = chrono::Local::now().date_naive();
        let dates = vec![
            today,
            today - chrono::Duration::days(1),
            // Gap here
            today - chrono::Duration::days(5),
            today - chrono::Duration::days(6),
        ];
        let streak = calculate_streak(&dates);
        assert_eq!(streak, 2); // Only counts consecutive days from today
    }

    #[test]
    #[should_panic(expected = "not yet implemented")]
    fn test_calculate_streak_empty() {
        let streak = calculate_streak(&[]);
        assert_eq!(streak, 0);
    }

    #[test]
    #[should_panic(expected = "not yet implemented")]
    fn test_calculate_streak_single_day() {
        let today = chrono::Local::now().date_naive();
        let dates = vec![today];
        let streak = calculate_streak(&dates);
        assert_eq!(streak, 1);
    }

    #[test]
    #[should_panic(expected = "not yet implemented")]
    fn test_merge_language_stats_multiple_projects() {
        let mut stats1 = HashMap::new();
        stats1.insert("Rust".to_string(), 1000);
        stats1.insert("JavaScript".to_string(), 500);

        let mut stats2 = HashMap::new();
        stats2.insert("Rust".to_string(), 2000);
        stats2.insert("Python".to_string(), 800);

        let merged = merge_language_stats(&[stats1, stats2]);

        // Should be sorted by count descending
        assert_eq!(
            merged,
            vec![
                ("Rust".to_string(), 3000),
                ("Python".to_string(), 800),
                ("JavaScript".to_string(), 500),
            ]
        );
    }

    #[test]
    #[should_panic(expected = "not yet implemented")]
    fn test_merge_language_stats_empty() {
        let merged = merge_language_stats(&[]);
        assert_eq!(merged, vec![]);
    }

    #[test]
    #[should_panic(expected = "not yet implemented")]
    fn test_merge_language_stats_single_project() {
        let mut stats = HashMap::new();
        stats.insert("Rust".to_string(), 1000);
        stats.insert("JavaScript".to_string(), 500);

        let merged = merge_language_stats(&[stats]);

        assert_eq!(
            merged,
            vec![
                ("Rust".to_string(), 1000),
                ("JavaScript".to_string(), 500),
            ]
        );
    }

    #[test]
    fn test_portfolio_stats_default() {
        let stats = PortfolioStats::default();
        assert_eq!(stats.total_loc, 0);
        assert_eq!(stats.project_count, 0);
        assert_eq!(stats.total_stars, 0);
        assert_eq!(stats.total_forks, 0);
        assert_eq!(stats.commit_streak, 0);
        assert_eq!(stats.total_commits, 0);
        assert!(stats.loc_by_language.is_empty());
        assert!(stats.most_active.is_empty());
        assert!(stats.languages.is_empty());
    }
}
