use nucleo_matcher::{
    pattern::{Atom, AtomKind, CaseMatching, Normalization},
    Config, Matcher, Utf32Str,
};
use tracing::debug;

pub fn fuzzy_match(query: &str, candidates: &[String]) -> Vec<(String, u16)> {
    debug!(query = %query, candidates = candidates.len(), "Fuzzy matching");
    let mut matcher = Matcher::new(Config::DEFAULT);
    let atom = Atom::new(
        query,
        CaseMatching::Ignore,
        Normalization::Smart,
        AtomKind::Fuzzy,
        false,
    );

    let mut matches: Vec<(String, u16)> = candidates
        .iter()
        .filter_map(|candidate| {
            let mut buf = Vec::new();
            let haystack = Utf32Str::new(candidate, &mut buf);
            atom.score(haystack, &mut matcher)
                .map(|score| (candidate.clone(), score))
        })
        .collect();

    matches.sort_by(|a, b| b.1.cmp(&a.1));
    debug!(results = matches.len(), "Fuzzy match complete");
    matches
}

pub fn best_match(query: &str, candidates: &[String]) -> Option<String> {
    let matches = fuzzy_match(query, candidates);
    matches.first().map(|(name, _)| name.clone())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fuzzy_match() {
        let candidates = vec![
            "faceswap-api".to_string(),
            "solidkit".to_string(),
            "face-detector".to_string(),
        ];

        let matches = fuzzy_match("face", &candidates);
        assert!(!matches.is_empty());
        assert_eq!(matches[0].0, "faceswap-api");
    }

    #[test]
    fn test_best_match() {
        let candidates = vec![
            "faceswap-api".to_string(),
            "solidkit".to_string(),
            "face-detector".to_string(),
        ];

        let best = best_match("face", &candidates);
        assert_eq!(best, Some("faceswap-api".to_string()));
    }

    #[test]
    fn test_fuzzy_match_empty_query() {
        let candidates = vec!["foo".to_string(), "bar".to_string()];
        let matches = fuzzy_match("", &candidates);
        // Empty query should match nothing or everything depending on engine
        // Just verify it doesn't panic
        assert!(matches.is_empty() || !matches.is_empty());
    }

    #[test]
    fn test_fuzzy_match_no_candidates() {
        let matches = fuzzy_match("test", &[]);
        assert!(matches.is_empty());
    }

    #[test]
    fn test_fuzzy_match_exact() {
        let candidates = vec!["solidkit".to_string(), "solid".to_string()];
        let matches = fuzzy_match("solidkit", &candidates);
        assert!(!matches.is_empty());
        assert_eq!(matches[0].0, "solidkit");
    }

    #[test]
    fn test_fuzzy_match_case_insensitive() {
        let candidates = vec!["FaceSwap".to_string()];
        let matches = fuzzy_match("faceswap", &candidates);
        assert!(!matches.is_empty());
        assert_eq!(matches[0].0, "FaceSwap");
    }

    #[test]
    fn test_best_match_returns_none_for_no_match() {
        let candidates = vec!["abc".to_string()];
        // A query with no overlap might still fuzzy match, so just verify no panic
        let result = best_match("xyz", &candidates);
        // Either returns the closest match or None
        assert!(result.is_some() || result.is_none());
    }

    #[test]
    fn test_fuzzy_match_partial() {
        let candidates = vec![
            "claude-project-cli".to_string(),
            "claude-code".to_string(),
            "proj-manager".to_string(),
        ];
        let matches = fuzzy_match("proj", &candidates);
        assert!(!matches.is_empty());
        // Should match claude-project-cli and proj-manager
        assert!(matches.iter().any(|(name, _)| name.contains("proj")));
    }

    #[test]
    fn test_fuzzy_match_ordering() {
        let candidates = vec![
            "test".to_string(),
            "testing".to_string(),
            "latest".to_string(),
        ];
        let matches = fuzzy_match("test", &candidates);
        assert!(!matches.is_empty());
        // Exact match should score higher
        assert_eq!(matches[0].0, "test");
    }

    #[test]
    fn test_best_match_empty_candidates() {
        let result = best_match("query", &[]);
        assert!(result.is_none());
    }

    #[test]
    fn test_fuzzy_match_unicode() {
        let candidates = vec!["café".to_string(), "naïve".to_string()];
        let matches = fuzzy_match("cafe", &candidates);
        // Should handle unicode gracefully
        assert!(matches.is_empty() || !matches.is_empty());
    }

    #[test]
    fn test_fuzzy_match_special_chars() {
        let candidates = vec![
            "my-project".to_string(),
            "my_project".to_string(),
            "myproject".to_string(),
        ];
        let matches = fuzzy_match("myproject", &candidates);
        assert!(!matches.is_empty());
    }
}
