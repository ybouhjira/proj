use nucleo_matcher::{
    pattern::{Atom, AtomKind, CaseMatching, Normalization},
    Config, Matcher, Utf32Str,
};

pub fn fuzzy_match(query: &str, candidates: &[String]) -> Vec<(String, u16)> {
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
}
