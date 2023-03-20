pub fn check(candidate: &str) -> bool {
    let lower = candidate.to_lowercase();
    lower.is_empty()
        || !lower
            .char_indices()
            .filter(|(_, c)| !c.is_whitespace())
            .filter(|(_, c)| c != &'-')
            .any(|(i, c)| lower.rfind(c) != Some(i))
}
