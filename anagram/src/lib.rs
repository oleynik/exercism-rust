use std::collections::{HashMap, HashSet};

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let map = decompose_word(word);
    let mut result: HashSet<&str> = HashSet::new();
    'next_word: for &w in possible_anagrams {
        if word.to_lowercase() == w.to_lowercase() {
            continue;
        }
        let mut clone = map.clone();
        for ch in w.to_lowercase().chars() {
            match clone.get(&ch) {
                Some(&count) => {
                    if count == 0 {
                        continue 'next_word;
                    } else {
                        clone.insert(ch, count - 1)
                    }
                }
                None => continue 'next_word,
            };
        }
        for &v in clone.values() {
            if v != 0 {
                continue 'next_word;
            }
        }
        result.insert(w);
    }
    result
}

fn decompose_word(word: &str) -> HashMap<char, u32> {
    let mut result: HashMap<char, u32> = HashMap::new();
    for ch in word.to_lowercase().chars() {
        match result.get(&ch) {
            Some(count) => result.insert(ch, count + 1),
            None => result.insert(ch, 1),
        };
    }
    result
}
