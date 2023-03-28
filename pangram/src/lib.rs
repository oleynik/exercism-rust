/// Determine whether a sentence is a pangram.
pub fn is_pangram(sentence: &str) -> bool {
    if sentence.len() < 26 {
        return false;
    }
    let mut abc = ('a'..='z').collect::<Vec<char>>();
    sentence
        .to_lowercase()
        .chars()
        .filter(|c| c.is_alphabetic())
        .for_each(|c| {
            abc.binary_search(&c).map(|idx| abc.remove(idx));
        });
    abc.is_empty()
}
