/// Determine whether a sentence is a pangram.
pub fn is_pangram(sentence: &str) -> bool {
    if sentence.len() < 26 {
        return false;
    }
    let lower = sentence.to_lowercase();
    ('a'..='z').all(|c| lower.contains(c))
}
