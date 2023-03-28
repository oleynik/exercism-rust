/// Determine whether a sentence is a pangram.
pub fn is_pangram(sentence: &str) -> bool {
    if sentence.len() < 26 {
        return false;
    }
    let mut abc = ('a'..='z').collect::<Vec<char>>();
    for ch in sentence.to_lowercase().chars() {
        if !ch.is_alphabetic() {
            continue;
        }
        abc.binary_search(&ch).map(|f| abc.remove(f));
    }
    abc.is_empty()
}
