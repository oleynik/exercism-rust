// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use std::collections::HashMap;
use std::option::Option;

fn count_words<'a>(words: &'a [&str]) -> HashMap<&'a str, u32> {
    let mut result: HashMap<&'a str, u32> = HashMap::new();
    for word in words {
        let option = result.get(*word);
        match option {
            Some(v) => {
                result.insert(word, v + 1);
            }
            None => {
                result.insert(word, 1);
            }
        }
    }
    result
}

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let magazine_map = count_words(magazine);
    let note_map = count_words(note);

    for (&word, &count) in note_map.iter() {
        match magazine_map.get(&word) {
            Some(&v) => {
                if v < count {
                    return false;
                }
            }
            None => (return false),
        }
    }
    true
}
